use crate::controllers::file_walker::walk;
use crate::models::enums::pos_path::InputSource;
use crate::models::structs::arguments::Arguments;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

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

    match arguments.get_path() {
        InputSource::Normal(path) => {
            if path.is_file() {
                let reader = match File::open(path) {
                    Ok(file) => Box::new(BufReader::new(file)),
                    Err(_) => {
                        eprintln!("File was either not found or inexistent.");
                        process::exit(1);
                    }
                };
                search_on_buffer(case_insensitive, reader, &or_terms, &and_terms, &ex_terms);
                return;
            }
            for file_path in walk(path) {
                let reader = match File::open(file_path) {
                    Ok(file) => Box::new(BufReader::new(file)),
                    Err(_) => {
                        eprintln!("File was either not found or inexistent.");
                        continue;
                    }
                };
                search_on_buffer(case_insensitive, reader, &or_terms, &and_terms, &ex_terms);
            }
        }
        InputSource::Stdin => {
            let reader: Box<dyn BufRead> = Box::new(BufReader::new(io::stdin()));
            search_on_buffer(case_insensitive, reader, &or_terms, &and_terms, &ex_terms);
        }
    };
}

fn search_on_buffer<'a>(
    case_insensitive: bool,
    reader: Box<dyn BufRead>,
    or_terms: &'a [String],
    and_terms: &'a [String],
    ex_terms: &'a [String],
) {
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

    for (original, _lower) in lines {
        println!("{}", original);
    }
}
