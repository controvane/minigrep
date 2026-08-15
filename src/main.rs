mod controllers;
mod models;

use std::env;
use controllers::arg_parser::parse_arguments;
use controllers::searcher::search_on_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = parse_arguments(args);
    
    println!("The file path is: {}", arguments.get_path());
    println!("And the search terms are {:?}", arguments.get_search_terms());
    
    search_on_file(arguments);
}
