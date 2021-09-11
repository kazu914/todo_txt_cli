pub struct Todo {
    pub content: String,
    pub priority: Option<String>,
    pub projects: Option<Vec<String>>,
    pub contexts: Option<Vec<String>>,
}

impl Todo {
    pub fn new(
        content: impl Into<String>,
        priority: Option<impl Into<String>>,
        projects: Option<Vec<impl Into<String>>>,
        contexts: Option<Vec<impl Into<String>>>,
    ) -> Todo {
        Todo {
            content: content.into(),
            priority: priority.map(Into::into),
            projects: projects
                .map(|projects| projects.into_iter().map(|project| project.into()).collect()),
            contexts: contexts
                .map(|contexts| contexts.into_iter().map(|context| context.into()).collect()),
        }
    }

    pub fn to_formatted_string(&self) -> String {
        let mut res: String = "".to_string();
        if let Some(priority) = &self.priority {
            res += &("(".to_string() + &priority.to_string() + ")");
        }
        res += &(" ".to_string() + &self.content.clone());
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
