mod repository;
use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let m = App::from(yaml).get_matches();
    if m.is_present("add") {
        println!("'todo'");
    }

    if let Some(m) = m.subcommand_matches("add") {
        add_content(m.value_of("content").unwrap());
    }
}

fn add_content(content: &str) {
    let file = repository::TodoFile::new("todo.txt");
    file.append(content);
    let contents = file.read();
    print!("{:?}", contents);
}
