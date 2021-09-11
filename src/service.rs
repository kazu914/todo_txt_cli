use super::constants::add_flags::*;
use super::helper::get_today;
use super::model::Todo;
use super::repository::TodoFile;
use clap::ArgMatches;

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
}
