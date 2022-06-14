use std::{error::Error, fs};

pub fn run(config: Configuration) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    search(&config.query, &contents)
        .iter_mut()
        .for_each(|result| println!("{result}"));

    Ok(())
}

pub struct Configuration {
    invoked_name: String,
    query: String,
    file_name: String,
}

impl Configuration {
    pub fn new(arguments: &[String]) -> Result<Configuration, &'static str> {
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

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    return content
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "l";
        let contents = "\
la peque
peque
peque
pasteleria
";
        assert_eq!(vec!["la peque", "pasteleria"], search(query, contents))
    }
}
