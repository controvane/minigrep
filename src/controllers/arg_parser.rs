use std::collections::HashMap;
use std::process;
use std::env;

use crate::models::enums::flag_value::FlagValue;
use crate::models::structs::arguments::Arguments;

pub fn parse_arguments(args: Vec<String>) -> Arguments{
    let mut arguments: HashMap<String, FlagValue> = HashMap::new();
    let required_flags = vec!["-eq"];
    let mut args_iter = args.iter().peekable();
    let mut ret_args = Arguments::new();
    ret_args.path = Some(env::current_dir().expect("Really? How did this happened? No cwd?").to_string());

    while let Some(arg) = args_iter.next() {
        if arg == "-f"{
            match args_iter.peek(){
                Some(value) => {
                    if value.starts_with('-'){
                        println!("Missing file path after -f flag.");
                        process::exit(1);
                    }
                    arguments.insert("-f".to_string(), FlagValue::Path(value.to_string()));
                    args_iter.next();
                },
                None => {
                    println!("File path required after -f flag.");
                    process::exit(1);
                }
            }
        }
        if arg == "-eq" {
            match args_iter.next(){
                Some(value) => {
                    if value.chars().next().unwrap_or(' ') == '-'{
                        println!("Missing search terms after -eq.");
                        process::exit(1);
                    }
                    let words = value.split('|').map(|s| s.to_string()).collect();
                    arguments.insert("-eq".to_string(), FlagValue::SearchTerms(words));
                },
                None => {
                    println!("Arguments for search required after -eq");
                    process::exit(1);
                }
            }
        }
        if arg == "-i" {
            arguments.insert("-i".to_string(),FlagValue::CaseSensitive(true));
        }
    }

    for req in required_flags {
        if !arguments.contains_key(req) {
            println!("Error, required flag {} is missing.", req);
            process::exit(1);
        }
    }
 
    if let Some(FlagValue::Path(value)) = arguments.get("-f"){
        ret_args.path = Some(value.clone());
    }

    ret_args.path = match arguments.get("-f"){
        Some(FlagValue::Path(value)) => Some(value.clone()),
        _ => panic!("This should not have happened"),
    };

    ret_args.search_terms = match arguments.get("-eq"){
        Some(FlagValue::SearchTerms(value)) => Some(value.clone()),
        _ => panic!("This should not have happened"),
    };

    if let Some(FlagValue::CaseSensitive(value)) = arguments.get("-i") {
        ret_args.case_insensitive = *value;
    }

    return ret_args;
}
