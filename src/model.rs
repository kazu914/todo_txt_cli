pub struct Todo {
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
            res += &"x "
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
                res += &(" @".to_string() + &context)
            }
        }

        res
    }

    pub fn complete(&mut self, completion_date: impl Into<String>) {
        self.is_completed = true;
        self.completion_date = Some(completion_date.into());
    }

    pub fn from_formatted_string(formatted_string: &str) -> Todo {
        let vec: Vec<&str> = formatted_string.split_whitespace().collect();
        let priority = vec[0];
        let content = vec[1];
        Todo {
            is_completed: false,
            completion_date: None,
            content: content.to_string(),
            priority: Some(priority.to_string()),
            creation_date: None,
            projects: None,
            contexts: None,
        }
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
        fn from_content() {
            let formatted_string = "todo test";
            let todo = super::Todo::from_formatted_string(formatted_string);
            assert_eq!(todo.content, "todo test");
        }

        #[test]
        fn from_priority_content() {
            let formatted_string = "(A) todo test";
            let todo = super::Todo::from_formatted_string(formatted_string);
            assert_eq!(todo.priority, Some("A".to_string()));
            assert_eq!(todo.content, "todo test");
        }
    }
}
