use std::process;
use std::fs;
use crate::models::structs::arguments::Arguments;

pub fn search_on_file(arguments: Arguments) {
    let file_contents = match fs::read_to_string(arguments.get_path()){
        Ok(value) => value,
        Err(_) => {
            println!("File was either not found or inexistent");
            process::exit(1);
        },
    };

    let terms: Vec<String> = arguments
        .get_search_terms()
        .iter()
        .map(|x|
            if arguments.get_case_insensitive() {
                return x.to_lowercase();
            }
            else{
                return x.clone();
            }
        )
        .collect();

    for line in file_contents.lines() {
        
        let op_line = if arguments.get_case_insensitive(){
            line.to_lowercase()
        }
        else{
            line.to_string()
        };

        let mut all_exist = vec![false; terms.len()];
        
        for (i,word) in terms.iter().enumerate() {
            if op_line.contains(word) {
                all_exist[i] = true;
            }
        }

        if all_exist.iter().all(|&x| x) {
            println!("{}", line);
        }
    }

}

