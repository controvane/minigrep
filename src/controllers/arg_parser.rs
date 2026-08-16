use std::collections::HashMap;
use std::env;
use std::process;

use crate::models::enums::flag_value::FlagValue;
use crate::models::structs::arguments::Arguments;

pub fn parse_arguments(args: Vec<String>) -> Arguments {
    let mut arguments: HashMap<String, FlagValue> = HashMap::new();
    let required_flags = vec!["-eq", "-c", "-ne"];
    let mut args_iter = args.iter().peekable();
    let mut ret_args = Arguments::new();
    ret_args.path = Some(env::current_dir().expect("Really? How did this happened? No cwd?"));

    while let Some(arg) = args_iter.next() {
        if arg == "-f" {
            match args_iter.peek() {
                Some(value) => {
                    if value.starts_with('-') {
                        println!("Missing file path after -f flag.");
                        process::exit(1);
                    }
                    arguments.insert(
                        "-f".to_string(),
                        FlagValue::Path(value.to_string().trim().into()),
                    );
                    args_iter.next();
                }
                Option::None => {
                    println!("File path required after -f flag.");
                    process::exit(1);
                }
            }
        }
        if arg == "-eq" {
            match args_iter.next() {
                Some(value) => {
                    if value.chars().next().unwrap_or(' ') == '-' {
                        println!("Missing search terms after -eq.");
                        process::exit(1);
                    }
                    let words = value.split('|').map(|s| s.to_string()).collect();
                    arguments.insert("-eq".to_string(), FlagValue::SearchTerms(words));
                }
                Option::None => {
                    println!("Arguments for search required after -eq");
                    process::exit(1);
                }
            }
        }
        if arg == "-c" {
            match args_iter.next() {
                Some(value) => {
                    if value.chars().next().unwrap_or(' ') == '-' {
                        println!("Missing search terms after -c.");
                        process::exit(1);
                    }
                    let words = value.split('|').map(|s| s.to_string()).collect();
                    arguments.insert("-c".to_string(), FlagValue::SearchTerms(words));
                }
                Option::None => {
                    println!("Arguments for search required after -c");
                    process::exit(1);
                }
            }
        }
        if arg == "-ne" {
            match args_iter.next() {
                Some(value) => {
                    if value.chars().next().unwrap_or(' ') == '-' {
                        println!("Missing exclude terms after -ne.");
                        process::exit(1);
                    }
                    let words = value.split('|').map(|s| s.to_string()).collect();
                    arguments.insert("-ne".to_string(), FlagValue::SearchTerms(words));
                }
                Option::None => {
                    println!("Arguments for exclusion required after -ne");
                    process::exit(1);
                }
            }
        }
        if arg == "-i" {
            arguments.insert("-i".to_string(), FlagValue::CaseSensitive(true));
        }
    }

    let mut any_exist: Vec<bool> = vec![false; 3];

    for (i, req) in required_flags.iter().enumerate() {
        if arguments.contains_key(*req) {
            any_exist[i] = true;
        }
    }

    if !any_exist.iter().any(|x| *x) {
        println!("At least one of the search arguments required: -eq, -ne or -c");
        process::exit(1);
    }

    if let Some(FlagValue::Path(value)) = arguments.get("-f") {
        ret_args.path = Some(value.clone());
    }

    if let Some(FlagValue::SearchTerms(value)) = arguments.get("-eq") {
        ret_args.and_search_terms = Some(value.clone())
    }

    if let Some(FlagValue::SearchTerms(value)) = arguments.get("-c") {
        ret_args.or_search_terms = Some(value.clone())
    }

    if let Some(FlagValue::SearchTerms(value)) = arguments.get("-ne") {
        ret_args.ex_search_terms = Some(value.clone())
    }

    if let Some(FlagValue::CaseSensitive(value)) = arguments.get("-i") {
        ret_args.case_insensitive = *value;
    }

    return ret_args;
}
