use clap::{load_yaml, App};
use todo_txt::constants::subcommands::*;
use todo_txt::service::TodoService;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let m = App::from(yaml).get_matches();
    let service = TodoService::new("todo.txt");

    if let Some(m) = m.subcommand_matches(ADD) {
        let todo_string = service.add_todo(m);
        println!("Created: {:?}", todo_string);
    }
}
