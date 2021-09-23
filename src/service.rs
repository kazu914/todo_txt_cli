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
        let todo = Todo::new(content, Some(creation_date), priority, projects, contexts);

        let todo_string = Converter::to_formatted_string(&todo);
        let _ = self.file.append(&todo_string);
        todo_string
    }

    pub fn complete_todo(&self, matches: &ArgMatches) -> String {
        let key: usize = matches.value_of_t(KEY).unwrap_or_else(|_| {
            println!("Error: Key should be integer");
            process::exit(1);
        });
        let mut lines = self.file.read();
        let todo_string = lines.get(key);
        if todo_string.is_none() {
            println!("Error: Couldn't find todo with key: {}", key);
            process::exit(1);
        }
        let completion_date: String = matches.value_of_t(DATE).unwrap_or_else(|_| get_today());
        if !is_valid_date(&completion_date) {
            println!(
                "Error: {} is invalid date format (YYYY-MM-DD)",
                completion_date
            );
            process::exit(1);
        }
        let mut todo = Converter::from_formatted_string(todo_string.unwrap(), Some(key));
        todo.complete(completion_date.as_str());
        lines[key] = Converter::to_formatted_string(&todo);
        self.file.overwrite(
            lines
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>()
                .as_ref(),
        );
        Converter::to_formatted_string(&todo)
    }

    pub fn list_todos(&self, matches: &ArgMatches) {
        let todos = self.file.read();
        let todo_list: Vec<Todo> = todos
            .iter()
            .enumerate()
            .map(|(index, todo)| Converter::from_formatted_string(todo, Some(index)))
            .collect();
        let presenter = Presenter::new(todo_list);

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
