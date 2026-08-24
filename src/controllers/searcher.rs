use crate::controllers::file_walker::walk;
use crate::models::enums::pos_path::InputSource;
use crate::models::structs::arguments::Arguments;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;

//three functions to apply the filters to and or and xor
fn matches_any(lower: &str, terms: &[String]) -> bool {
    return terms.iter().any(|p| lower.contains(p));
}

fn matches_all(lower: &str, terms: &[String]) -> bool {
    return terms.iter().all(|p| lower.contains(p));
}

fn matches_none(lower: &str, terms: &[String]) -> bool {
    return !terms.iter().any(|p| lower.contains(p));
}

fn line_matches(
    lower: &str,
    or_terms: &[String],
    and_terms: &[String],
    ex_terms: &[String],
) -> bool {
    return (or_terms.is_empty() || matches_any(lower, or_terms))
        && (and_terms.is_empty() || matches_all(lower, and_terms))
        && (ex_terms.is_empty() || matches_none(lower, ex_terms));
}

pub fn search(arguments: Arguments) {
    let case_insensitive = arguments.get_case_insensitive();
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
    let or_terms: Vec<String> = arguments
        .get_search_terms_or()
        .iter()
        .map(|x| {
            if case_insensitive {
                return x.to_lowercase();
            } else {
                return x.clone();
            }
        })
        .collect();

    let and_terms: Vec<String> = arguments
        .get_search_terms_and()
        .iter()
        .map(|x| {
            if case_insensitive {
                return x.to_lowercase();
            } else {
                return x.clone();
            }
        })
        .collect();

    let ex_terms: Vec<String> = arguments
        .get_search_terms_ex()
        .iter()
        .map(|x| {
            if case_insensitive {
                return x.to_lowercase();
            } else {
                return x.clone();
            }
        })
        .collect();

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
                    case_insensitive,
                    reader,
                    &or_terms,
                    &and_terms,
                    &ex_terms,
                    before_lines,
                    after_lines,
                    num_lines,
                ));
                return;
            }
            //Do search over all files on the selected directory on multiple threads
            //Quickends searhces on directories
            walk(path, &file_types).par_bridge().for_each(|file_path| {
                let reader = match File::open(&file_path) {
                    Ok(file) => Box::new(BufReader::new(file)),
                    Err(_) => {
                        eprintln!("File was either not found or inexistent.");
                        return;
                    }
                };
                let mut lines = search_on_buffer(
                    case_insensitive,
                    reader,
                    &or_terms,
                    &and_terms,
                    &ex_terms,
                    before_lines,
                    after_lines,
                    num_lines,
                )
                .peekable();
                if lines.peek().is_none() {
                    return;
                }
                let stdout = io::stdout();
                let mut out = stdout.lock();
                writeln!(out, "{}: ", file_path.display()).unwrap();
                for line in lines {
                    writeln!(out, "\t{}", line).unwrap();
                }
            });
        }
        //Arm that reads what was piped from another command
        InputSource::Stdin => {
            let reader: Box<dyn BufRead> = Box::new(BufReader::new(io::stdin()));
            print_found_lines(search_on_buffer(
                case_insensitive,
                reader,
                &or_terms,
                &and_terms,
                &ex_terms,
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
    for line in found_lines {
        println!("{}", line);
    }
}

//This is the function that does the search_on_buffer
//Receives a buffer of the content of the file and compares it
fn search_on_buffer(
    case_insensitive: bool,
    reader: Box<dyn BufRead>,
    or_terms: &[String],
    and_terms: &[String],
    ex_terms: &[String],
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
            let lower = if case_insensitive {
                line.to_lowercase()
            } else {
                line.clone()
            };

            if line_matches(&lower, or_terms, and_terms, ex_terms) {
                let mut block = Vec::new();

                //A fresh group after skipped lines needs a separator. The
                //first group gets no leading separator, and a match inside an
                //ongoing after-window is a continuation, so skip it then too.
                if after_left == 0 && gap && emitted_any && (after_lines + before_lines) > 0 {
                    block.push("\t.\n\t.\n\t.".to_string());
                }

                //Emit the before-context we have been holding.
                block.extend(before_buf.drain(..).map(|(i, elem)| {
                    if numerate_lines {
                        return format!("{}: {}", i + 1, elem.trim()).to_string();
                    }
                    return elem.trim().to_string();
                }));

                let pushable_line = if numerate_lines {
                    format!("{}: {}", index + 1, line.trim()).to_string()
                } else {
                    line.trim().to_string()
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
                    format!("{}: {}", index + 1, line.trim()).to_string()
                } else {
                    line.trim().to_string()
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
