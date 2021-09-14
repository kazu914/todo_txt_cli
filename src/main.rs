use clap::{load_yaml, App};
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::BufReader};
use todo_txt::constants::subcommands::*;
use todo_txt::service::TodoService;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    default_file_name: String,
}

fn main() {
    let config: Config = read_config();
    println!("{:?}", config);
    let yaml = load_yaml!("cli.yaml");
    let m = App::from(yaml).get_matches();
    let service = TodoService::new("todo.txt");

    if let Some(m) = m.subcommand_matches(ADD) {
        let todo_string = service.add_todo(m);
        println!("Created: {:?}", todo_string);
    }

    if let Some(m) = m.subcommand_matches(DONE) {
        let todo_string = service.complete_todo(m);
        println!("Completed: {:?}", todo_string);
    }

    if let Some(_) = m.subcommand_matches(LIST) {
        service.list_todos();
    }
}

fn read_config() -> Config {
    let config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("config.json")
        .unwrap();
    let reader = BufReader::new(config_file);
    serde_json::from_reader(reader).unwrap()
}
