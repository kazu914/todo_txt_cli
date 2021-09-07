use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let m = App::from(yaml).get_matches();
    if m.is_present("add") {
        println!("'todo'");
    }

    if let Some(m) = m.subcommand_matches("add") {
        println!("Adding todo: {}", m.value_of("content").unwrap());
    }
}
