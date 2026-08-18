mod controllers;
mod models;

use controllers::arg_parser::parse_arguments;
use controllers::searcher::search;
use models::enums::pos_path::InputSource;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = parse_arguments(args);

    match arguments.get_path() {
        InputSource::Normal(path) => {
            println!("The search file is: {}", path.to_string_lossy());
        }
        InputSource::Stdin => println!("The search stream is: (piped)"),
    }
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

    search(arguments);
}
