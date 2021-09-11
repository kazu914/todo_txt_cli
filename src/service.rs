use super::helper::get_today;
use super::model::Todo;
use super::repository::TodoFile;
use clap::ArgMatches;
const CONTENT: &str = "content";
const PRIORITY: &str = "priority";
const PROJECTS: &str = "projects";
const CONTEXTS: &str = "contexts";

pub struct TodoService {
    file: TodoFile,
}

impl TodoService {
    pub fn new(path: &str) -> TodoService {
        TodoService {
            file: TodoFile::new(path),
        }
    }
    pub fn add_todo(&self, matches: &ArgMatches) {
        let content: &str = matches.value_of(CONTENT).unwrap();
        let creation_date: String = get_today();
        let priority: Option<&str> = matches.value_of(PRIORITY);
        let projects: Option<Vec<String>> = matches.values_of_t(PROJECTS).ok();
        let contexts: Option<Vec<String>> = matches.values_of_t(CONTEXTS).ok();
        let todo = Todo::new(content, Some(creation_date), priority, projects, contexts);
        &self.file.append(&todo.to_formatted_string());
    }
}
