pub struct Todo {
    pub content: String,
    pub priority: String,
    pub projects: Vec<String>,
    pub contexts: Vec<String>,
}

impl Todo {
    pub fn new(
        content: impl Into<String>,
        priority: impl Into<String>,
        projects: Vec<impl Into<String>>,
        contexts: Vec<impl Into<String>>,
    ) -> Todo {
        Todo {
            content: content.into(),
            priority: priority.into(),
            projects: projects.into_iter().map(|project| project.into()).collect(),
            contexts: contexts.into_iter().map(|context| context.into()).collect(),
        }
    }
}
