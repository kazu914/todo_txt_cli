pub struct Todo {
    pub content: String,
    pub creation_date: Option<String>,
    pub priority: Option<String>,
    pub projects: Option<Vec<String>>,
    pub contexts: Option<Vec<String>>,
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

        if let Some(priority) = &self.priority {
            res += &("(".to_string() + &priority.to_string() + ") ");
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
            let todo: super::Todo =
                super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(todo.to_formatted_string(), "content")
        }

        #[test]
        fn priority_content() {
            let content: &str = "content";
            let creation_date: Option<&str> = None;
            let priority: Option<&str> = Some("A");
            let projects: Option<Vec<String>> = None;
            let contexts: Option<Vec<String>> = None;
            let todo: super::Todo =
                super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(todo.to_formatted_string(), "(A) content")
        }

        #[test]
        fn creation_date_content() {
            let content: &str = "content";
            let creation_date: Option<&str> = Some("2000-1-1");
            let priority: Option<&str> = None;
            let projects: Option<Vec<String>> = None;
            let contexts: Option<Vec<String>> = None;
            let todo: super::Todo =
                super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(todo.to_formatted_string(), "2000-1-1 content")
        }

        #[test]
        fn priority_creation_date_content() {
            let content: &str = "content";
            let creation_date: Option<&str> = Some("2000-1-1");
            let priority: Option<&str> = Some("A");
            let projects: Option<Vec<String>> = None;
            let contexts: Option<Vec<String>> = None;
            let todo: super::Todo =
                super::Todo::new(content, creation_date, priority, projects, contexts);
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
            let todo: super::Todo =
                super::Todo::new(content, creation_date, priority, projects, contexts);
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

            let todo: super::Todo =
                super::Todo::new(content, creation_date, priority, projects, contexts);
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

            let todo: super::Todo =
                super::Todo::new(content, creation_date, priority, projects, contexts);
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

            let todo: super::Todo =
                super::Todo::new(content, creation_date, priority, projects, contexts);
            assert_eq!(
                todo.to_formatted_string(),
                "(A) 2000-1-1 content +projectA +projectB @contextA @contextB"
            )
        }
    }
}
