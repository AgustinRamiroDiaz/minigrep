use std::{env, error::Error, fs, process};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let configuration = Configuration::new(&arguments).unwrap_or_else(|err| {
        println!("Error creating configuration: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(configuration) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: Configuration) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    println!("With text:\n{}", contents);

    Ok(())
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
