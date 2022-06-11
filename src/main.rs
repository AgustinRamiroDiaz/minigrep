use std::{env, fs};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = parse_configuration(&arguments);
    println!(
        "Called as '{}', searching for '{}' in file '{}'",
        config.invoked_name, config.query, config.file_name
    );

    let content =
        fs::read_to_string(config.file_name).expect("I shouldn't panic for reading a file 🙃");

    println!("Contets of '{}':\n{}", config.file_name, content);
}

struct configuration<'a> {
    invoked_name: &'a String,
    query: &'a String,
    file_name: &'a String,
}

fn parse_configuration(arguments: &[String]) -> configuration {
    configuration {
        invoked_name: &arguments[0],
        query: &arguments[1],
        file_name: &arguments[2],
    }
}
