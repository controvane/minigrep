use crate::controllers::file_walker::walk;
use crate::models::enums::pos_path::InputSource;
use crate::models::enums::search_mode::SearchMode;
use crate::models::structs::arguments::Arguments;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::process;

fn line_matches(
    line: &str,
    or_terms: &Option<SearchMode>,
    and_terms: &Option<SearchMode>,
    ex_terms: &Option<SearchMode>,
) -> bool {
    return (match or_terms {
        Some(terms) => terms.matches_any(line),
        None => true,
    }) && (match and_terms {
        Some(terms) => terms.matches_all(line),
        None => true,
    }) && (match ex_terms {
        Some(terms) => terms.matches_none(line),
        None => true,
    });
}

pub fn search(arguments: Arguments) {
    let before_lines = arguments.get_before_lines();
    let after_lines = arguments.get_after_lines();
    let num_lines = arguments.get_numerate_lines();
    let file_types: Vec<&str> = arguments
        .get_file_types()
        .iter()
        .map(|elem| elem.as_str())
        .collect();

    //pulling the terms from arguments and transforming to lower case
    //for case insensitiveness
    let or_terms = arguments.get_search_terms_or();

    let and_terms = arguments.get_search_terms_and();

    let ex_terms = arguments.get_search_terms_ex();

    //Because the input can come from two distinct sources
    //We match with this enum
    match arguments.get_path() {
        InputSource::Normal(path) => {
            //normal search over one file
            if path.is_file() {
                let reader = match File::open(path) {
                    Ok(file) => Box::new(BufReader::new(file)),
                    Err(_) => {
                        eprintln!("File was either not found or inexistent.");
                        process::exit(1);
                    }
                };
                print_found_lines(search_on_buffer(
                    reader,
                    or_terms,
                    and_terms,
                    ex_terms,
                    before_lines,
                    after_lines,
                    num_lines,
                ));
                return;
            }
            //Do search over all files on the selected directory on multiple threads
            //Quickends searhces on directories
            walk(path, &file_types)
                .into_par_iter()
                .for_each(|file_path| {
                    let reader = match File::open(&file_path) {
                        Ok(file) => Box::new(BufReader::new(file)),
                        Err(_) => {
                            eprintln!("File was either not found or inexistent.");
                            return;
                        }
                    };
                    let mut lines = search_on_buffer(
                        reader,
                        or_terms,
                        and_terms,
                        ex_terms,
                        before_lines,
                        after_lines,
                        num_lines,
                    )
                    .peekable();
                    if lines.peek().is_none() {
                        return;
                    }
                    let stdout = io::stdout();
                    let mut out = BufWriter::new(stdout.lock());
                    if let Err(e) = writeln!(out, "{}: ", file_path.display()) {
                        if e.kind() == std::io::ErrorKind::BrokenPipe {
                            process::exit(0);
                        }
                        eprintln!("well ... this should not have happened: {}.", e);
                        process::exit(1);
                    }
                    for line in lines {
                        if let Err(e) = writeln!(out, "\t{}", line.trim()) {
                            if e.kind() == std::io::ErrorKind::BrokenPipe {
                                process::exit(0);
                            }
                            eprintln!("well ... this should not have happened: {}.", e);
                            process::exit(1);
                        };
                    }
                    if let Err(e) = out.flush() {
                        if e.kind() == std::io::ErrorKind::BrokenPipe {
                            return;
                        }
                        eprintln!("well ... this should not have happened: {}.", e);
                        process::exit(1);
                    };
                });
        }
        //Arm that reads what was piped from another command
        InputSource::Stdin => {
            let reader: Box<dyn BufRead> = Box::new(BufReader::new(io::stdin()));
            print_found_lines(search_on_buffer(
                reader,
                or_terms,
                and_terms,
                ex_terms,
                before_lines,
                after_lines,
                num_lines,
            ));
        }
    };
}

//Cause I did not wanted to use the same code in two different places
//It just loops and prints over a list of strings.
fn print_found_lines(found_lines: impl Iterator<Item = String>) {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    for line in found_lines {
        if let Err(e) = writeln!(out, "{}", line.trim()) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return;
            }
            eprintln!("well ... this should not have happened: {}.", e);
            process::exit(1);
        };
    }
    if let Err(e) = out.flush() {
        if e.kind() == std::io::ErrorKind::BrokenPipe {
            return;
        }
        eprintln!("well ... this should not have happened: {}.", e);
        process::exit(1);
    };
}

//This is the function that does the search_on_buffer
//Receives a buffer of the content of the file and compares it
fn search_on_buffer(
    reader: Box<dyn BufRead>,
    or_terms: &Option<SearchMode>,
    and_terms: &Option<SearchMode>,
    ex_terms: &Option<SearchMode>,
    before_lines: u16,
    after_lines: u16,
    numerate_lines: bool,
) -> impl Iterator<Item = String> {
    let before = before_lines as usize;
    let mut before_buf: VecDeque<(usize, String)> = VecDeque::with_capacity(before);
    let mut after_left: u16 = 0;
    let mut gap = false;
    let mut emitted_any = false;

    return reader
        .lines()
        .map_while(Result::ok)
        .enumerate()
        .flat_map(move |(index, line)| {
            if line_matches(&line, or_terms, and_terms, ex_terms) {
                let mut block = Vec::new();

                //A fresh group after skipped lines needs a separator. The
                //first group gets no leading separator, and a match inside an
                //ongoing after-window is a continuation, so skip it then too.
                //of course, if no after or before lines exist, this
                //separator does not either.
                if after_left == 0 && gap && emitted_any && (after_lines + before_lines) > 0 {
                    block.push("\t.\n\t.\n\t.".to_string());
                }

                //Emit the before-context we have been holding.
                block.extend(before_buf.drain(..).map(|(i, elem)| {
                    if numerate_lines {
                        return format!("{}:\t{}", i + 1, elem.trim());
                    }
                    return elem;
                }));

                let pushable_line = if numerate_lines {
                    format!("{}:\t{}", index + 1, line.trim())
                } else {
                    line
                };

                //And the matched line itself.
                block.push(pushable_line);

                after_left = after_lines;
                gap = false;
                emitted_any = true;
                return block;
            }

            if after_left > 0 {
                let pushable_line = if numerate_lines {
                    format!("{}:\t{}", index + 1, line.trim())
                } else {
                    line
                };
                //After-context: emit it and count down.
                after_left -= 1;
                return vec![pushable_line];
            }

            //Not a match and not after-context: hold it as a before-context
            //candidate, dropping the oldest when the buffer overflows.
            before_buf.push_back((index, line));
            if before_buf.len() > before {
                before_buf.pop_front();
                gap = true;
            }
            return Vec::new();
        });
}
