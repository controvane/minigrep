use is_terminal::IsTerminal;
use std::collections::HashMap;
use std::io;
use std::process;

use crate::models::enums::flag_value::FlagValue;
use crate::models::enums::pos_path::InputSource;
use crate::models::structs::arguments::Arguments;

pub fn parse_arguments(args: Vec<String>) -> Arguments {
    let mut arguments: HashMap<String, FlagValue> = HashMap::new();
    let required_flags = vec!["-eq", "-c", "-ne"];
    let mut args_iter = args.iter().peekable();
    let mut ret_args = Arguments::new();

    //loop through the argument and flags and organize em
    while let Some(arg) = args_iter.next() {
        //Optional file or dir path
        if arg == "-f" {
            match args_iter.peek() {
                Some(value) => {
                    if value.starts_with('-') {
                        eprintln!("Missing file path after -f flag.");
                        process::exit(1);
                    }
                    arguments.insert(
                        "-f".to_string(),
                        FlagValue::Path(value.to_string().trim().into()),
                    );
                    args_iter.next();
                }
                Option::None => {
                    eprintln!("File path required after -f flag.");
                    process::exit(1);
                }
            }
        }
        // And terms
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
        //or terms
        if arg == "-c" {
            match args_iter.next() {
                Some(value) => {
                    if value.chars().next().unwrap_or(' ') == '-' {
                        eprintln!("Missing search terms after -c.");
                        process::exit(1);
                    }
                    let words = value.split('|').map(|s| s.to_string()).collect();
                    arguments.insert("-c".to_string(), FlagValue::SearchTerms(words));
                }
                Option::None => {
                    eprintln!("Arguments for search required after -c");
                    process::exit(1);
                }
            }
        }
        //excluded terms
        if arg == "-ne" {
            match args_iter.next() {
                Some(value) => {
                    if value.chars().next().unwrap_or(' ') == '-' {
                        eprintln!("Missing exclude terms after -ne.");
                        process::exit(1);
                    }
                    let words = value.split('|').map(|s| s.to_string()).collect();
                    arguments.insert("-ne".to_string(), FlagValue::SearchTerms(words));
                }
                Option::None => {
                    eprintln!("Arguments for exclusion required after -ne");
                    process::exit(1);
                }
            }
        }
        //case insensiteveness
        if arg == "-i" {
            arguments.insert("-i".to_string(), FlagValue::CaseSensitive(true));
        }
        //amount for lines included after the matched line
        if arg == "-a" {
            match args_iter.next() {
                Some(a) => {
                    let after: u16 = match a.parse() {
                        Ok(value) => value,
                        Err(_) => {
                            eprintln!("A positive number is needed for lines after.");
                            process::exit(1);
                        }
                    };
                    arguments.insert("-a".to_string(), FlagValue::ExtraLines(after));
                }
                Option::None => {
                    eprintln!("An amount of lines after the match needed for -a");
                    process::exit(1);
                }
            }
        }
        //amount for lines included after the matched line
        if arg == "-b" {
            match args_iter.next() {
                Some(b) => {
                    let before: u16 = match b.parse() {
                        Ok(value) => value,
                        Err(_) => {
                            eprintln!("A positive number is needed for lines before.");
                            process::exit(1);
                        }
                    };
                    arguments.insert("-b".to_string(), FlagValue::ExtraLines(before));
                }
                Option::None => {
                    eprintln!("An amount of lines before the match needed for -b");
                    process::exit(1);
                }
            }
        }
        //help command
        if arg == "-h" || arg == "--help" {
            print_helper();
            process::exit(0);
        }
    }

    let mut any_exist: Vec<bool> = vec![false; 3];

    if !arguments.contains_key("-f") && !io::stdin().is_terminal() {
        ret_args.path = InputSource::Stdin;
    }

    //check that at least one of the search terms was defined, else crash
    for (i, req) in required_flags.iter().enumerate() {
        if arguments.contains_key(*req) {
            any_exist[i] = true;
        }
    }

    if !any_exist.iter().any(|x| *x) {
        eprintln!("At least one of the search arguments required: -eq, -ne or -c");
        process::exit(1);
    }

    //with the -f if above this builds the Arguments that will be sent
    if let Some(FlagValue::Path(value)) = arguments.get("-f") {
        ret_args.path = InputSource::Normal(value.clone());
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

    if let Some(FlagValue::ExtraLines(value)) = arguments.get("-a") {
        ret_args.after_lines = *value;
    }

    if let Some(FlagValue::ExtraLines(value)) = arguments.get("-b") {
        ret_args.before_lines = *value;
    }

    return ret_args;
}

//prints all of the help text

fn print_helper() {
    println!("minigrep is a tiny grep clone project in rust.");
    println!(
        "When using to search follow any of the search flags with a list of arguments in quotations."
    );
    println!("Divide the arguments with pipes ('|') inside the quotations.");
    println!("This are the possible arguments:");
    println!(
        "-f: Optional. In case it is not passed it will use cwd. It is the path of the file or directory where the search should take place"
    );
    println!("-i: Forces case insensitivity throughout the search");
    println!("-c: Search terms with or. Any line that has at least one of this will be shown.");
    println!(
        "-eq: Search terms with and. All terms described with this flag have to be present for the line to be shown."
    );
    println!(
        "-ne: Excluded terms. Lines with this terms will be excluded from the output. Even if other search criteria found them."
    );
    println!("-a: Number of lines to print after each match as context. E.g: -a 2");
    println!("-b: Number of lines to print before each match as context. E.g: -b 2");
}
