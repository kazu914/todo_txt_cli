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

    pub fn to_formatted_string(&self) -> String {
        let mut res: String = "(".to_string() + &self.priority.clone() + ")";
        res += &(" ".to_string() + &self.content.clone());
        for project in &self.projects {
            res += &(" +".to_string() + &project.to_string())
        }
        for context in &self.contexts {
            res += &(" @".to_string() + &context.to_string())
        }

        return res;
    }
}
