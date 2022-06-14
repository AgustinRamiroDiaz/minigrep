use minigrep::{run, Configuration};
use std::{env, process};

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
