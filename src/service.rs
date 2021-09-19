use super::constants::add_flags::*;
use super::constants::done_flags::*;
use super::constants::list_flags::*;
use super::helper::{get_today, is_valid_date};
use super::model::Todo;
use super::repository::TodoFile;
use clap::ArgMatches;
use cli_table::{print_stdout, Cell, Style, Table};
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
        let projects: Option<Vec<String>> = matches.values_of_t(PROJECTS).ok();
        let contexts: Option<Vec<String>> = matches.values_of_t(CONTEXTS).ok();
        let todo = Todo::new(content, Some(creation_date), priority, projects, contexts);

        let todo_string = todo.to_formatted_string();
        &self.file.append(&todo_string);
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
        let mut todo = Todo::from_formatted_string(todo_string.unwrap(), Some(key));
        todo.complete(completion_date.as_str());
        lines[key] = todo.to_formatted_string();
        self.file
            .overwrite(&lines.iter().map(String::as_str).collect());
        todo.to_formatted_string()
    }

    pub fn list_todos(&self, matches: &ArgMatches) {
        let todos = self.file.read();
        match matches.value_of(FORMAT).unwrap_or_default() {
            "table" => {
                let todo_list: Vec<Todo> = todos
                    .iter()
                    .enumerate()
                    .map(|(index, todo)| Todo::from_formatted_string(todo, Some(index)))
                    .collect();

                let table_formats: Vec<Vec<String>> = todo_list
                    .iter()
                    .map(|todo| todo.to_table_format())
                    .collect();

                let table = table_formats
                    .table()
                    .title(vec![
                        "key".cell().bold(true),
                        "completed?".cell().bold(true),
                        "priority".cell().bold(true),
                        "completion date".cell().bold(true),
                        "creation date".cell().bold(true),
                        "projects".cell().bold(true),
                        "contexts".cell().bold(true),
                        "content".cell().bold(true),
                    ])
                    .bold(true);
                let _ = print_stdout(table);
            }
            _ => {
                let todos = self.file.read();
                for (i, todo) in todos.iter().enumerate() {
                    println!("{}: {}", i, todo);
                }
            }
        }
    }
}
