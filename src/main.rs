mod controllers;
mod models;

use controllers::arg_parser::parse_arguments;
use controllers::searcher::search_on_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = parse_arguments(args);

    println!(
        "The search file is: {}",
        arguments.get_path().to_string_lossy()
    );
    println!(
        "The or search terms are: {:?}",
        arguments.get_search_terms_or()
    );
    println!(
        "The and search terms are: {:?}",
        arguments.get_search_terms_and()
    );
    println!(
        "The excluded terms are: {:?}",
        arguments.get_search_terms_ex()
    );

    search_on_file(arguments);
}
