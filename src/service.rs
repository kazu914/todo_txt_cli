use super::constants::add_flags::*;
use super::constants::done_flags::*;
use super::constants::list_flags::*;
use super::converter::Converter;
use super::helper::{get_today, is_valid_date};
use super::model::Todo;
use super::presenter::Presenter;
use super::repository::TodoFile;
use clap::ArgMatches;
use std::process;

pub struct TodoService {
    file: TodoFile,
}

impl TodoService {
    pub fn new(path: &str) -> TodoService {
        TodoService {
            file: TodoFile::new(path),
        }
    }
    pub fn add_todo(&self, matches: &ArgMatches) -> String {
        let content: &str = matches.value_of(CONTENT).unwrap();
        let creation_date: String = get_today();
        let priority: Option<&str> = matches.value_of(PRIORITY);
        let projects: Option<String> = matches.value_of_t(PROJECTS).ok();
        let contexts: Option<String> = matches.value_of_t(CONTEXTS).ok();
        let todo =
            Converter::from_argments(content, Some(creation_date), priority, projects, contexts)
                .unwrap_or_else(|e| {
                    eprintln!("{}", e);
                    process::exit(1);
                });

        let todo_string = Converter::to_formatted_string(&todo);
        let _ = self.file.append(&todo_string);
        todo_string
    }

    pub fn complete_todo(&self, matches: &ArgMatches) -> String {
        let key: usize = matches.value_of_t(KEY).unwrap_or_else(|_| {
            println!("Error: Key should be integer");
            process::exit(1);
        });
        let todo_string = self.file.get_todo_with_key(key);
        let completion_date: String = matches.value_of_t(DATE).unwrap_or_else(|_| get_today());
        if !is_valid_date(&completion_date) {
            println!(
                "Error: {} is invalid date format (YYYY-MM-DD)",
                completion_date
            );
            process::exit(1);
        }
        let mut todo = Converter::from_formatted_string(&todo_string, Some(key));
        todo.complete(completion_date.as_str());
        self.file
            .update_todo(key, &Converter::to_formatted_string(&todo));
        Converter::to_formatted_string(&todo)
    }

    pub fn delete_todo(&self, matches: &ArgMatches) {
        let key: usize = matches.value_of_t(KEY).unwrap_or_else(|_| {
            println!("Error: Key should be integer");
            process::exit(1);
        });
        if self.file.check_if_exist(key) {
            println!("Error: Couldn't find todo with key: {}", key);
            process::exit(1);
        }
        self.file.remove(key);
    }

    pub fn list_todos(&self, matches: &ArgMatches) {
        let todo_lines = self.file.read();
        let todos: Vec<Todo> = todo_lines
            .iter()
            .enumerate()
            .map(|(index, todo)| Converter::from_formatted_string(todo, Some(index)))
            .collect();
        let presenter = Presenter::new(todos);

        match matches.value_of(FORMAT).unwrap_or_default() {
            "table" => {
                presenter.pring_table();
            }
            _ => {
                presenter.print();
            }
        }
    }
}
