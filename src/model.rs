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
        content: String,
        creation_date: Option<String>,
        priority: Option<String>,
        projects: Option<Vec<String>>,
        contexts: Option<Vec<String>>,
    ) -> Todo {
        let key: Option<usize> = None;
        let is_completed = false;
        let completion_date: Option<String> = None;
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
    pub fn empty() -> Todo {
        Todo {
            key: None,
            is_completed: false,
            completion_date: None,
            content: "".to_string(),
            priority: None,
            creation_date: None,
            projects: None,
            contexts: None,
        }
    }

    pub fn complete(&mut self, completion_date: impl Into<String>) {
        self.is_completed = true;
        self.completion_date = Some(completion_date.into());
    }
}

impl Todo {
    pub fn key(&self) -> &Option<usize> {
        &self.key
    }
    pub fn is_completed(&self) -> &bool {
        &self.is_completed
    }
    pub fn priority(&self) -> &Option<String> {
        &self.priority
    }
    pub fn completion_date(&self) -> &Option<String> {
        &self.completion_date
    }
    pub fn creation_date(&self) -> &Option<String> {
        &self.creation_date
    }
    pub fn projects(&self) -> &Option<Vec<String>> {
        &self.projects
    }
    pub fn contexts(&self) -> &Option<Vec<String>> {
        &self.contexts
    }
    pub fn content(&self) -> &String {
        &self.content
    }
    pub fn set_key(&mut self, key: Option<usize>) {
        self.key = key;
    }
    pub fn set_is_completed(&mut self, is_completed: bool) {
        self.is_completed = is_completed;
    }
    pub fn set_priority(&mut self, priority: Option<String>) {
        self.priority = priority;
    }
    pub fn set_completion_date(&mut self, completion_date: Option<String>) {
        self.completion_date = completion_date;
    }
    pub fn set_creation_date(&mut self, creation_date: Option<String>) {
        self.creation_date = creation_date;
    }
    pub fn set_projects(&mut self, projects: Option<Vec<String>>) {
        self.projects = projects;
    }
    pub fn set_contexts(&mut self, contexts: Option<Vec<String>>) {
        self.contexts = contexts;
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

#[cfg(test)]
mod tests {
    use super::Todo;

    mod complete {
        #[test]
        fn complete() {
            let content: String = "content".to_string();
            let creation_date: Option<String> = None;
            let priority: Option<String> = None;
            let projects: Option<Vec<String>> = None;
            let contexts: Option<Vec<String>> = None;
            let mut todo = super::Todo::new(content, creation_date, priority, projects, contexts);
            todo.complete("2000-1-2");
            assert!(todo.is_completed);
            assert_eq!(todo.completion_date, Some("2000-1-2".to_string()));
        }
    }
}
