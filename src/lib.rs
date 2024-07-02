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
    let mut results = Vec::new();
    println!("All File Paths:====================");
    explore_dir(&config.query, &config.filepath, &mut results)?;

    println!("Results:====================");
    for result in results {
        println!("{}", result);
    }
    Ok(())
}

fn explore_dir(query: &str, dir: &str, results: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    if !Path::new(dir).exists() {
        return Err(format!("{} does not exist", dir).into());
    }

    let mut stack = vec![dir.to_string()];

    while let Some(current_dir) = stack.pop() {
        if !Path::new(&current_dir).exists() {
            return Err(format!("{} does not exist", current_dir).into());
        }

        let entries = fs::read_dir(&current_dir)?;
        for entry in entries {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let file_path = entry.file_name();
            let full_path_dir = format!("{}/{}", current_dir, file_path.to_string_lossy());

            if file_type.is_dir() {
                stack.push(full_path_dir.clone());
            } else {
                println!("File: {:?}", full_path_dir);
                if full_path_dir.contains(query) {
                    results.push(full_path_dir);
                }
            }
        }
    }
    Ok(())
}
