use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use clap::{App, Arg, SubCommand};
use serde::{Serialize, Deserialize};

// Define a structure for serializing/deserializing environment variables
#[derive(Serialize, Deserialize)]
struct EnvVars(HashMap<String, String>);

fn save_env(name: &str) -> Result<(), io::Error> {
    // Save all existing environment variables
    let saved_env_vars = EnvVars(env::vars().collect());

    // Serialize the environment variables to JSON
    let serialized_env = serde_json::to_string(&saved_env_vars)?;

    // Write the JSON to a file
    let mut file = File::create(name)?;
    file.write_all(serialized_env.as_bytes())?;

    println!("Environment variables saved to '{}'", name);

    Ok(())
}

fn load_env(name: &str) -> Result<(), io::Error> {
    // Read the JSON file and deserialize it
    let file = File::open(name)?;
    let saved_env_vars: EnvVars = serde_json::from_reader(file)?;

    // Compare the serialized environment variables with the current environment
    for (key, value) in &saved_env_vars.0 {
        match env::var(&key) {
            Ok(current_value) => {
                if current_value != *value {
                    // Environment variable has changed; update it
                    println!("export {}=\"{}\"", key, value);
                }
            }
            Err(_) => {
                // Environment variable doesn't exist; set it
                println!("export {}=\"{}\"", key, value);
            }
        }
    }

    // Unset environment variables that are not in the serialized data
    for (key, _) in env::vars() {
        if !saved_env_vars.0.contains_key(&key) {
            println!("unset {}", key);
        }
    }

    eprintln!("Environment variables loaded and updated from '{}'", name);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("envs")
        .version("1.0")
        .author("Your Name")
        .about("Save and load environment variables")
        .subcommand(
            SubCommand::with_name("save")
                .about("Save environment variables")
                .arg(Arg::with_name("name").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("load")
                .about("Load environment variables")
                .arg(Arg::with_name("name").required(true).index(1)),
        )
        .get_matches();

    if let Some(save_matches) = matches.subcommand_matches("save") {
        let name = save_matches.value_of("name").unwrap();
        save_env(name)?;
    } else if let Some(load_matches) = matches.subcommand_matches("load") {
        let name = load_matches.value_of("name").unwrap();
        if let Err(err) = load_env(name) {
            eprintln!("Error loading and updating environment variables: {}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}
