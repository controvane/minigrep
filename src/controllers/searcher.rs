use crate::controllers::file_walker::walk;
use crate::models::enums::pos_path::InputSource;
use crate::models::structs::arguments::Arguments;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;

//three functions to apply the filters to and or and xor
fn apply_or_filters(
    lines: impl Iterator<Item = (String, String)>,
    terms: &[String],
) -> impl Iterator<Item = (String, String)> {
    return lines.filter(move |(_orig, lower)| terms.iter().any(|p| lower.contains(p)));
}

fn apply_and_filters(
    lines: impl Iterator<Item = (String, String)>,
    terms: &[String],
) -> impl Iterator<Item = (String, String)> {
    return lines.filter(move |(_orig, lower)| terms.iter().all(|p| lower.contains(p)));
}

fn apply_exclusions(
    lines: impl Iterator<Item = (String, String)>,
    terms: &[String],
) -> impl Iterator<Item = (String, String)> {
    return lines.filter(move |(_orig, lower)| !terms.iter().any(|p| lower.contains(p)));
}

pub fn search(arguments: Arguments) {
    let case_insensitive = arguments.get_case_insensitive();

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
                ));
                return;
            }
            //Do search over all files on the selected directory on multiple threads
            //Quickends searhces on directories
            walk(path).par_bridge().for_each(|file_path| {
                let reader = match File::open(&file_path) {
                    Ok(file) => Box::new(BufReader::new(file)),
                    Err(_) => {
                        eprintln!("File was either not found or inexistent.");
                        return;
                    }
                };
                let lines =
                    search_on_buffer(case_insensitive, reader, &or_terms, &and_terms, &ex_terms);
                if lines.is_empty() {
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
            ));
        }
    };
}

//Cause I did not wanted to use the same code in two different places
//It just loops and prints over a list of strings.
fn print_found_lines(found_lines: Vec<String>) {
    for line in found_lines {
        println!("{}", line);
    }
}

//This is the function that does the search_on_buffer
//Receives a buffer of the content of the file and compares it
//with each of the possible search terms in chain
fn search_on_buffer<'a>(
    case_insensitive: bool,
    reader: Box<dyn BufRead>,
    or_terms: &'a [String],
    and_terms: &'a [String],
    ex_terms: &'a [String],
) -> Vec<String> {
    let base_lines = reader.lines().map_while(Result::ok);
    let mut lines: Box<dyn Iterator<Item = (String, String)> + 'a> = if case_insensitive {
        Box::new(base_lines.map(|line| {
            let lower = line.to_lowercase();
            return (line, lower);
        }))
    } else {
        Box::new(base_lines.map(|line| {
            let clone = line.clone();
            return (line, clone);
        }))
    };

    if or_terms.len() > 0 {
        lines = Box::new(apply_or_filters(lines, or_terms));
    }

    if and_terms.len() > 0 {
        lines = Box::new(apply_and_filters(lines, and_terms));
    }

    if ex_terms.len() > 0 {
        lines = Box::new(apply_exclusions(lines, ex_terms));
    }

    return lines.map(|(original, _lower)| original).collect();
}
