use std::{env, fs, process};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let Configuration {
        invoked_name,
        query,
        file_name,
    } = Configuration::new(&arguments).unwrap_or_else(|err| {
        println!("Error creating configuration: {}", err);
        process::exit(1);
    });
    println!(
        "Called as '{}', searching for '{}' in file '{}'",
        invoked_name, query, file_name
    );

    let content = fs::read_to_string(&file_name).expect("I shouldn't panic for reading a file 🙃");

    println!("Contets of '{}':\n{}", file_name, content);
}

struct Configuration {
    invoked_name: String,
    query: String,
    file_name: String,
}

impl Configuration {
    fn new(arguments: &[String]) -> Result<Configuration, &'static str> {
        if arguments.len() < 3 {
            return Err("Not enough arguments");
        }

        Ok(Configuration {
            invoked_name: arguments[0].clone(),
            query: arguments[1].clone(),
            file_name: arguments[2].clone(),
        })
    }
}
