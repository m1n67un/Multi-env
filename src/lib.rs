mod handlers;
mod models;

use std::env;

use clap::Parser;
use handlers::load::load_yaml_to_env;
use models::args::Args;

pub fn set_ok(file_type: &str) {
    let args = Args::parse();
    let environment = args.branch.as_str();
    
    match file_type {
        "env" => {
            dotenv::from_filename(format!(".env.{}", &environment)).ok();
            dotenv::dotenv().ok();
        },
        "yaml" | "yml" => {
            let common_path = format!("common.yaml");
            load_yaml_to_env(&common_path).expect("The specified file does not exist.");

            let file_path = format!("{}.{}", &environment, file_type);
            load_yaml_to_env(&file_path).expect("The specified file does not exist.");
        },
        _ => {
            println!("No configuration has been specified. As the default value is 'env', the program will execute using the 'env' configuration.");
        }
    }
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}