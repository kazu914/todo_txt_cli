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
        let priority: &str = matches.value_of(PRIORITY).unwrap();
        let projects: Vec<&str> = matches.values_of(PROJECTS).unwrap().collect();
        let contexts: Vec<&str> = matches.values_of(CONTEXTS).unwrap().collect();
        let todo = Todo::new(content, priority, projects, contexts);

        &self.file.append(&todo.to_formatted_string());
    }
}
