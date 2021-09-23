pub struct Converter {}
use super::helper::{is_context, is_project, is_valid_date};
use super::model::Todo;

impl Converter {
    pub fn from_formatted_string(formatted_string: &str, key: Option<usize>) -> Todo {
        let mut is_completed: bool = false;
        let mut priority: Option<String> = None;
        let mut completion_date: Option<String> = None;
        let mut creation_date: Option<String> = None;
        let mut content: String;
        let mut projects: Option<Vec<String>> = None;
        let mut contexts: Option<Vec<String>> = None;
        let mut iter = formatted_string.split_whitespace();
        let mut value: &str = iter.next().unwrap();

        if value.eq("x") {
            is_completed = true;
            value = iter.next().unwrap();
        }

        if value.starts_with('(') {
            priority = Some(value.chars().nth(1).unwrap().to_string());
            value = iter.next().unwrap();
        }
        if is_valid_date(value) {
            let date1: Option<String> = Some(value.to_string());
            value = iter.next().unwrap();
            if is_valid_date(value) {
                let date2: Option<String> = Some(value.to_string());
                value = iter.next().unwrap();
                completion_date = date1;
                creation_date = date2;
            } else if is_completed {
                completion_date = date1;
            } else {
                creation_date = date1;
            }
        }

        content = value.to_string();

        let mut projects_vec: Vec<String> = [].to_vec();
        let mut contexts_vec: Vec<String> = [].to_vec();
        for val in iter {
            if is_project(val) {
                projects_vec.push(val[1..].to_string());
            } else if is_context(val) {
                contexts_vec.push(val[1..].to_string());
            } else {
                content += &(" ".to_string() + val);
            }
        }

        if !projects_vec.is_empty() {
            projects = Some(projects_vec);
        }
        if !contexts_vec.is_empty() {
            contexts = Some(contexts_vec);
        }
        let mut todo = Todo::empty();
        todo.set_key(key);
        todo.set_is_completed(is_completed);
        todo.set_completion_date(completion_date);
        todo.set_content(content);
        todo.set_priority(priority);
        todo.set_creation_date(creation_date);
        todo.set_projects(projects);
        todo.set_contexts(contexts);
        todo
    }
    pub fn to_formatted_string(todo: Todo) -> String {
        let mut res: String = "".to_string();

        if *todo.is_completed() {
            res += "x "
        }

        if let Some(priority) = &todo.priority() {
            res += &("(".to_string() + &priority.to_string() + ") ");
        }

        if let Some(completion_date) = &todo.completion_date() {
            res += &(completion_date.to_string() + " ");
        }

        if let Some(creation_date) = &todo.creation_date() {
            res += &(creation_date.to_string() + " ");
        }

        res += todo.content();

        if let Some(projects) = &todo.projects() {
            for project in projects {
                res += &(" +".to_string() + project)
            }
        }

        if let Some(contexts) = &todo.contexts() {
            for context in contexts {
                res += &(" @".to_string() + context)
            }
        }

        res
    }
}
#[cfg(test)]
mod tests {
    use super::{Converter, Todo};
    mod to_formatted_string {
        #[test]
        fn content() {
            let content: &str = "content";
            let creation_date: Option<&str> = None;
            let priority: Option<&str> = None;
            let projects: Option<String> = None;
            let contexts: Option<String> = None;
            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(super::Converter::to_formatted_string(todo), "content")
        }

        #[test]
        fn priority_content() {
            let content: &str = "content";
            let creation_date: Option<&str> = None;
            let priority: Option<&str> = Some("A");
            let projects: Option<String> = None;
            let contexts: Option<String> = None;
            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(super::Converter::to_formatted_string(todo), "(A) content")
        }

        #[test]
        fn creation_date_content() {
            let content: &str = "content";
            let creation_date: Option<&str> = Some("2000-1-1");
            let priority: Option<&str> = None;
            let projects: Option<String> = None;
            let contexts: Option<String> = None;
            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(
                super::Converter::to_formatted_string(todo),
                "2000-1-1 content"
            )
        }

        #[test]
        fn priority_creation_date_content() {
            let content: &str = "content";
            let creation_date: Option<&str> = Some("2000-1-1");
            let priority: Option<&str> = Some("A");
            let projects: Option<String> = None;
            let contexts: Option<String> = None;
            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(
                super::Converter::to_formatted_string(todo),
                "(A) 2000-1-1 content"
            )
        }

        #[test]
        fn content_projects() {
            let content: &str = "content";
            let creation_date: Option<&str> = None;
            let priority: Option<&str> = None;
            let projects: Option<String> = Some("projectA,projectB".to_string());
            let contexts: Option<String> = None;
            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(
                super::Converter::to_formatted_string(todo),
                "content +projectA +projectB"
            )
        }

        #[test]
        fn content_contexts() {
            let content: &str = "content";
            let creation_date: Option<&str> = None;
            let priority: Option<&str> = None;
            let projects: Option<String> = None;
            let contexts: Option<String> = Some("contextA,contextB".to_string());

            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(
                super::Converter::to_formatted_string(todo),
                "content @contextA @contextB"
            )
        }

        #[test]
        fn content_projects_contexts() {
            let content: &str = "content";
            let creation_date: Option<&str> = None;
            let priority: Option<&str> = None;
            let projects: Option<String> = Some("projectA,projectB".to_string());
            let contexts: Option<String> = Some("contextA,contextB".to_string());

            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(
                super::Converter::to_formatted_string(todo),
                "content +projectA +projectB @contextA @contextB"
            )
        }

        #[test]
        fn priority_creation_date_content_projects_contexts() {
            let content: &str = "content";
            let creation_date: Option<&str> = Some("2000-1-1");
            let priority: Option<&str> = Some("A");
            let projects: Option<String> = Some("projectA,projectB".to_string());
            let contexts: Option<String> = Some("contextA,contextB".to_string());

            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(
                super::Converter::to_formatted_string(todo),
                "(A) 2000-1-1 content +projectA +projectB @contextA @contextB"
            )
        }

        #[test]
        fn complete_priority_creation_date_content_projects_contexts() {
            let content: &str = "content";
            let creation_date: Option<&str> = Some("2000-1-1");
            let priority: Option<&str> = Some("A");
            let projects: Option<String> = Some("projectA,projectB".to_string());
            let contexts: Option<String> = Some("contextA,contextB".to_string());

            let mut todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            todo.complete("2000-1-2");

            assert_eq!(
                super::Converter::to_formatted_string(todo),
                "x (A) 2000-1-2 2000-1-1 content +projectA +projectB @contextA @contextB"
            )
        }
    }
    mod from_formatted_string {
        #[test]
        fn from_flag() {
            let formatted_string = "x todo text";
            let todo = super::Converter::from_formatted_string(formatted_string, None);
            assert!(todo.is_completed());
        }

        #[test]
        fn from_priority() {
            let formatted_string = "(A) todo text";
            let todo = super::Converter::from_formatted_string(formatted_string, None);
            assert!(!todo.is_completed());
            assert_eq!(todo.priority(), &Some("A".to_string()));
            assert_eq!(todo.content(), &"todo text".to_string());
        }

        #[test]
        fn from_flag_priority() {
            let formatted_string = "x (A) todo text";
            let todo = super::Converter::from_formatted_string(formatted_string, None);
            assert!(todo.is_completed());
            assert_eq!(todo.priority(), &Some("A".to_string()));
            assert_eq!(todo.content(), &"todo text".to_string());
        }

        #[test]
        fn from_priority_creation_date() {
            let formatted_string = "(A) 2000-1-1 todo text";
            let todo = super::Converter::from_formatted_string(formatted_string, None);
            assert!(!todo.is_completed());
            assert_eq!(todo.priority(), &Some("A".to_string()));
            assert_eq!(todo.completion_date(), &None);
            assert_eq!(todo.creation_date(), &Some("2000-1-1".to_string()));
            assert_eq!(todo.content(), &"todo text".to_string());
        }

        #[test]
        fn from_flag_priority_completion_date() {
            let formatted_string = "x (A) 2000-1-1 todo text";
            let todo = super::Converter::from_formatted_string(formatted_string, None);
            assert!(todo.is_completed());
            assert_eq!(todo.priority(), &Some("A".to_string()));
            assert_eq!(todo.completion_date(), &Some("2000-1-1".to_string()));
            assert_eq!(todo.creation_date(), &None);
            assert_eq!(todo.content(), &"todo text".to_string());
        }

        #[test]
        fn from_flag_priority_completion_date_creation_date() {
            let formatted_string = "x (A) 2000-1-1 1999-12-31 todo text";
            let todo = super::Converter::from_formatted_string(formatted_string, None);
            assert!(todo.is_completed());
            assert_eq!(todo.priority(), &Some("A".to_string()));
            assert_eq!(todo.completion_date(), &Some("2000-1-1".to_string()));
            assert_eq!(todo.creation_date(), &Some("1999-12-31".to_string()));
            assert_eq!(todo.content(), &"todo text".to_string());
        }

        #[test]
        fn from_content_projects() {
            let formatted_string = "todo text +projectA +projectB";
            let todo = super::Converter::from_formatted_string(formatted_string, None);
            assert!(!todo.is_completed());
            assert_eq!(todo.priority(), &None);
            assert_eq!(todo.completion_date(), &None);
            assert_eq!(todo.creation_date(), &None);
            assert_eq!(todo.content(), &"todo text".to_string());
            assert_eq!(
                todo.projects(),
                &Some(vec!["projectA".to_string(), "projectB".to_string()])
            );
        }

        #[test]
        fn from_content_contexts() {
            let formatted_string = "todo text @contextA @contextB";
            let todo = super::Converter::from_formatted_string(formatted_string, None);
            assert!(!todo.is_completed());
            assert_eq!(todo.priority(), &None);
            assert_eq!(todo.completion_date(), &None);
            assert_eq!(todo.creation_date(), &None);
            assert_eq!(todo.content(), &"todo text".to_string());
            assert_eq!(
                todo.contexts(),
                &Some(vec!["contextA".to_string(), "contextB".to_string()])
            );
        }

        #[test]
        fn from_content_projects_contexts() {
            let formatted_string = "todo text +projectA +projectB @contextA @contextB";
            let todo = super::Converter::from_formatted_string(formatted_string, None);
            assert!(!todo.is_completed());
            assert_eq!(todo.priority(), &None);
            assert_eq!(todo.completion_date(), &None);
            assert_eq!(todo.creation_date(), &None);
            assert_eq!(todo.content(), &"todo text".to_string());
            assert_eq!(
                todo.projects(),
                &Some(vec!["projectA".to_string(), "projectB".to_string()])
            );
            assert_eq!(
                todo.contexts(),
                &Some(vec!["contextA".to_string(), "contextB".to_string()])
            );
        }
    }
}
