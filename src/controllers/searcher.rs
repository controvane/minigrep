use crate::controllers::file_walker::walk;
use crate::models::enums::pos_path::InputSource;
use crate::models::structs::arguments::Arguments;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn apply_or_filters(
    lines: impl Iterator<Item = (String, String)>,
    terms: Vec<String>,
) -> impl Iterator<Item = (String, String)> {
    return lines.filter(move |(_orig, lower)| terms.iter().any(|p| lower.contains(p)));
}

fn apply_and_filters(
    lines: impl Iterator<Item = (String, String)>,
    terms: Vec<String>,
) -> impl Iterator<Item = (String, String)> {
    return lines.filter(move |(_orig, lower)| terms.iter().all(|p| lower.contains(p)));
}

fn apply_exclusions(
    lines: impl Iterator<Item = (String, String)>,
    terms: Vec<String>,
) -> impl Iterator<Item = (String, String)> {
    return lines.filter(move |(_orig, lower)| !terms.iter().any(|p| lower.contains(p)));
}

pub fn search(arguments: Arguments) {
    match arguments.get_path() {
        InputSource::Normal(path) => {
            if path.is_file() {
                let reader = match File::open(path) {
                    Ok(file) => Box::new(BufReader::new(file)),
                    Err(_) => {
                        println!("File was either not found or inexistent.");
                        process::exit(1);
                    }
                };
                search_on_buffer(&arguments, reader);
                return;
            }
            for file_path in walk(path) {
                let reader = match File::open(file_path) {
                    Ok(file) => Box::new(BufReader::new(file)),
                    Err(_) => {
                        println!("File was either not found or inexistent.");
                        continue;
                    }
                };
                search_on_buffer(&arguments, reader);
            }
        }
        InputSource::Stdin => {
            let reader: Box<dyn BufRead> = Box::new(BufReader::new(io::stdin()));
            search_on_buffer(&arguments, reader);
        }
    };
}

fn search_on_buffer(arguments: &Arguments, reader: Box<dyn BufRead>) {
    let or_terms: Vec<String> = arguments
        .get_search_terms_or()
        .iter()
        .map(|x| {
            if arguments.get_case_insensitive() {
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
            if arguments.get_case_insensitive() {
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
            if arguments.get_case_insensitive() {
                return x.to_lowercase();
            } else {
                return x.clone();
            }
        })
        .collect();

    let base_lines = reader.lines().map_while(Result::ok);
    let mut lines: Box<dyn Iterator<Item = (String, String)>> = if arguments.get_case_insensitive()
    {
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

    if arguments.get_search_terms_or().len() > 0 {
        lines = Box::new(apply_or_filters(lines, or_terms));
    }

    if arguments.get_search_terms_and().len() > 0 {
        lines = Box::new(apply_and_filters(lines, and_terms));
    }

    if arguments.get_search_terms_ex().len() > 0 {
        lines = Box::new(apply_exclusions(lines, ex_terms));
    }

    for (original, _lower) in lines {
        println!("{}", original);
    }
}
