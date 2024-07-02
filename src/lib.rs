use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { query, filepath })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    explore_dir(&config.filepath)?;
    Ok(())
}

fn explore_dir(dir: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(dir).exists() {
        return Err(format!("{} does not exist", dir).into());
    }
    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let file_path = entry.file_name();
        let full_path_dir = format!("{}/{}", dir, file_path.to_string_lossy());

        if file_type.is_dir() {
            println!("Dir: {:?}", full_path_dir);
            explore_dir(&full_path_dir)?;
        } else {
            println!("File: {:?}", full_path_dir);
        }
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "rs";
        let contents = "\
src/main.rs
poem.txt
";

        assert_eq!(vec!["src/main.rs"], search(query, contents));
    }
}
