use super::helper::{is_context, is_project, is_valid_date};

pub struct Todo {
    key: Option<usize>,
    is_completed: bool,
    completion_date: Option<String>,
    content: String,
    creation_date: Option<String>,
    priority: Option<String>,
    projects: Option<Vec<String>>,
    contexts: Option<Vec<String>>,
}

impl Todo {
    pub fn new(
        content: impl Into<String>,
        creation_date: Option<impl Into<String>>,
        priority: Option<impl Into<String>>,
        projects: Option<Vec<impl Into<String>>>,
        contexts: Option<Vec<impl Into<String>>>,
    ) -> Todo {
        Todo {
            key: None,
            is_completed: false,
            completion_date: None,
            content: content.into(),
            priority: priority.map(Into::into),
            creation_date: creation_date.map(Into::into),
            projects: projects
                .map(|projects| projects.into_iter().map(|project| project.into()).collect()),
            contexts: contexts
                .map(|contexts| contexts.into_iter().map(|context| context.into()).collect()),
        }
    }

    pub fn to_formatted_string(&self) -> String {
        let mut res: String = "".to_string();

        if self.is_completed {
            res += "x "
        }

        if let Some(priority) = &self.priority {
            res += &("(".to_string() + &priority.to_string() + ") ");
        }

        if let Some(completion_date) = &self.completion_date {
            res += &(completion_date.to_string() + " ");
        }

        if let Some(creation_date) = &self.creation_date {
            res += &(creation_date.to_string() + " ");
        }

        res += &self.content;

        if let Some(projects) = &self.projects {
            for project in projects {
                res += &(" +".to_string() + project)
            }
        }

        if let Some(contexts) = &self.contexts {
            for context in contexts {
                res += &(" @".to_string() + context)
            }
        }

        res
    }

    pub fn complete(&mut self, completion_date: impl Into<String>) {
        self.is_completed = true;
        self.completion_date = Some(completion_date.into());
    }

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

        Todo {
            key,
            is_completed,
            completion_date,
            content,
            priority,
            creation_date,
            projects,
            contexts,
        }
    }

    pub fn to_table_format(&self) -> Vec<String> {
        let key = if self.key.is_some() {
            self.key.unwrap().to_string()
        } else {
            "-".to_string()
        };
        let is_copleted_string = if self.is_completed {
            "x".to_string()
        } else {
            "-".to_string()
        };

        let priority_string = self.priority.clone().unwrap_or_else(|| "-".to_string());
        let completion_date_string = self
            .completion_date
            .clone()
            .unwrap_or_else(|| "-".to_string());
        let creation_date_string = self
            .creation_date
            .clone()
            .unwrap_or_else(|| "-".to_string());

        let projects_string = self
            .projects
            .clone()
            .unwrap_or_else(|| vec!["-".to_string()])
            .join(" ");
        let contexts_string = self
            .contexts
            .clone()
            .unwrap_or_else(|| vec!["-".to_string()])
            .join(" ");

        let content_string = self.content.clone();

        vec![
            key,
            is_copleted_string,
            priority_string,
            completion_date_string,
            creation_date_string,
            projects_string,
            contexts_string,
            content_string,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::Todo;

    mod to_formatted_string {
        #[test]
        fn content() {
            let content: &str = "content";
            let creation_date: Option<&str> = None;
            let priority: Option<&str> = None;
            let projects: Option<Vec<String>> = None;
            let contexts: Option<Vec<String>> = None;
            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(todo.to_formatted_string(), "content")
        }

        #[test]
        fn priority_content() {
            let content: &str = "content";
            let creation_date: Option<&str> = None;
            let priority: Option<&str> = Some("A");
            let projects: Option<Vec<String>> = None;
            let contexts: Option<Vec<String>> = None;
            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(todo.to_formatted_string(), "(A) content")
        }

        #[test]
        fn creation_date_content() {
            let content: &str = "content";
            let creation_date: Option<&str> = Some("2000-1-1");
            let priority: Option<&str> = None;
            let projects: Option<Vec<String>> = None;
            let contexts: Option<Vec<String>> = None;
            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(todo.to_formatted_string(), "2000-1-1 content")
        }

        #[test]
        fn priority_creation_date_content() {
            let content: &str = "content";
            let creation_date: Option<&str> = Some("2000-1-1");
            let priority: Option<&str> = Some("A");
            let projects: Option<Vec<String>> = None;
            let contexts: Option<Vec<String>> = None;
            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(todo.to_formatted_string(), "(A) 2000-1-1 content")
        }

        #[test]
        fn content_projects() {
            let content: &str = "content";
            let creation_date: Option<&str> = None;
            let priority: Option<&str> = None;
            let projects: Option<Vec<String>> =
                Some(vec!["projectA".to_string(), "projectB".to_string()]);
            let contexts: Option<Vec<String>> = None;
            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(todo.to_formatted_string(), "content +projectA +projectB")
        }

        #[test]
        fn content_contexts() {
            let content: &str = "content";
            let creation_date: Option<&str> = None;
            let priority: Option<&str> = None;
            let projects: Option<Vec<String>> = None;
            let contexts: Option<Vec<String>> =
                Some(vec!["contextA".to_string(), "contextB".to_string()]);

            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(todo.to_formatted_string(), "content @contextA @contextB")
        }

        #[test]
        fn content_projects_contexts() {
            let content: &str = "content";
            let creation_date: Option<&str> = None;
            let priority: Option<&str> = None;
            let projects: Option<Vec<String>> =
                Some(vec!["projectA".to_string(), "projectB".to_string()]);
            let contexts: Option<Vec<String>> =
                Some(vec!["contextA".to_string(), "contextB".to_string()]);

            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(
                todo.to_formatted_string(),
                "content +projectA +projectB @contextA @contextB"
            )
        }

        #[test]
        fn priority_creation_date_content_projects_contexts() {
            let content: &str = "content";
            let creation_date: Option<&str> = Some("2000-1-1");
            let priority: Option<&str> = Some("A");
            let projects: Option<Vec<String>> =
                Some(vec!["projectA".to_string(), "projectB".to_string()]);
            let contexts: Option<Vec<String>> =
                Some(vec!["contextA".to_string(), "contextB".to_string()]);

            let todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(
                todo.to_formatted_string(),
                "(A) 2000-1-1 content +projectA +projectB @contextA @contextB"
            )
        }

        #[test]
        fn complete_priority_creation_date_content_projects_contexts() {
            let content: &str = "content";
            let creation_date: Option<&str> = Some("2000-1-1");
            let priority: Option<&str> = Some("A");
            let projects: Option<Vec<String>> =
                Some(vec!["projectA".to_string(), "projectB".to_string()]);
            let contexts: Option<Vec<String>> =
                Some(vec!["contextA".to_string(), "contextB".to_string()]);

            let mut todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            todo.complete("2000-1-2");

            assert_eq!(
                todo.to_formatted_string(),
                "x (A) 2000-1-2 2000-1-1 content +projectA +projectB @contextA @contextB"
            )
        }
    }

    mod complete {
        #[test]
        fn complete() {
            let content: &str = "content";
            let creation_date: Option<&str> = None;
            let priority: Option<&str> = None;
            let projects: Option<Vec<String>> = None;
            let contexts: Option<Vec<String>> = None;
            let mut todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            todo.complete("2000-1-2");
            assert!(todo.is_completed);
            assert_eq!(todo.completion_date, Some("2000-1-2".to_string()));
        }
    }

    mod from_formatted_string {
        #[test]
        fn from_flag() {
            let formatted_string = "x todo text";
            let todo = super::Todo::from_formatted_string(formatted_string, None);
            assert!(todo.is_completed);
        }

        #[test]
        fn from_priority() {
            let formatted_string = "(A) todo text";
            let todo = super::Todo::from_formatted_string(formatted_string, None);
            assert!(!todo.is_completed);
            assert_eq!(todo.priority, Some("A".to_string()));
            assert_eq!(todo.content, "todo text".to_string());
        }

        #[test]
        fn from_flag_priority() {
            let formatted_string = "x (A) todo text";
            let todo = super::Todo::from_formatted_string(formatted_string, None);
            assert!(todo.is_completed);
            assert_eq!(todo.priority, Some("A".to_string()));
            assert_eq!(todo.content, "todo text".to_string());
        }

        #[test]
        fn from_priority_creation_date() {
            let formatted_string = "(A) 2000-1-1 todo text";
            let todo = super::Todo::from_formatted_string(formatted_string, None);
            assert!(!todo.is_completed);
            assert_eq!(todo.priority, Some("A".to_string()));
            assert_eq!(todo.completion_date, None);
            assert_eq!(todo.creation_date, Some("2000-1-1".to_string()));
            assert_eq!(todo.content, "todo text".to_string());
        }

        #[test]
        fn from_flag_priority_completion_date() {
            let formatted_string = "x (A) 2000-1-1 todo text";
            let todo = super::Todo::from_formatted_string(formatted_string, None);
            assert!(todo.is_completed);
            assert_eq!(todo.priority, Some("A".to_string()));
            assert_eq!(todo.completion_date, Some("2000-1-1".to_string()));
            assert_eq!(todo.creation_date, None);
            assert_eq!(todo.content, "todo text".to_string());
        }

        #[test]
        fn from_flag_priority_completion_date_creation_date() {
            let formatted_string = "x (A) 2000-1-1 1999-12-31 todo text";
            let todo = super::Todo::from_formatted_string(formatted_string, None);
            assert!(todo.is_completed);
            assert_eq!(todo.priority, Some("A".to_string()));
            assert_eq!(todo.completion_date, Some("2000-1-1".to_string()));
            assert_eq!(todo.creation_date, Some("1999-12-31".to_string()));
            assert_eq!(todo.content, "todo text".to_string());
        }

        #[test]
        fn from_content_projects() {
            let formatted_string = "todo text +projectA +projectB";
            let todo = super::Todo::from_formatted_string(formatted_string, None);
            assert!(!todo.is_completed);
            assert_eq!(todo.priority, None);
            assert_eq!(todo.completion_date, None);
            assert_eq!(todo.creation_date, None);
            assert_eq!(todo.content, "todo text".to_string());
            assert_eq!(
                todo.projects,
                Some(vec!["projectA".to_string(), "projectB".to_string()])
            );
        }

        #[test]
        fn from_content_contexts() {
            let formatted_string = "todo text @contextA @contextB";
            let todo = super::Todo::from_formatted_string(formatted_string, None);
            assert!(!todo.is_completed);
            assert_eq!(todo.priority, None);
            assert_eq!(todo.completion_date, None);
            assert_eq!(todo.creation_date, None);
            assert_eq!(todo.content, "todo text".to_string());
            assert_eq!(
                todo.contexts,
                Some(vec!["contextA".to_string(), "contextB".to_string()])
            );
        }

        #[test]
        fn from_content_projects_contexts() {
            let formatted_string = "todo text +projectA +projectB @contextA @contextB";
            let todo = super::Todo::from_formatted_string(formatted_string, None);
            assert!(!todo.is_completed);
            assert_eq!(todo.priority, None);
            assert_eq!(todo.completion_date, None);
            assert_eq!(todo.creation_date, None);
            assert_eq!(todo.content, "todo text".to_string());
            assert_eq!(
                todo.projects,
                Some(vec!["projectA".to_string(), "projectB".to_string()])
            );
            assert_eq!(
                todo.contexts,
                Some(vec!["contextA".to_string(), "contextB".to_string()])
            );
        }
    }

    mod to_table_format {
        #[test]
        fn to_table() {
            let formatted_string = "todo text +projectA +projectB @contextA @contextB";
            let todo = super::Todo::from_formatted_string(formatted_string, None);
            let table_format: Vec<String> = vec![
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "projectA projectB".to_string(),
                "contextA contextB".to_string(),
                "todo text".to_string(),
            ];
            assert_eq!(todo.to_table_format(), table_format);
        }
    }
}
