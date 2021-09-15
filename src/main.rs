use clap::{load_yaml, App};
use todo_txt::config::Config;
use todo_txt::constants::subcommands::*;
use todo_txt::service::TodoService;

fn main() {
    let config = Config::new();
    let yaml = load_yaml!("cli.yaml");
    let m = App::from(yaml).get_matches();
    let service = TodoService::new(&config.default_file_name);

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
