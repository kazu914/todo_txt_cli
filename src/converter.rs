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
    pub fn to_formatted_string(todo: &Todo) -> String {
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
    use test_case::test_case;

    #[test_case("content", None, None, None, None, None, "content")]
    #[test_case("content", None, Some("A"), None, None, None, "(A) content")]
    #[test_case(
        "content",
        Some("2000-1-1"),
        Some("A"),
        None,
        None,
        None,
        "(A) 2000-1-1 content"
    )]
    #[test_case(
        "content",
        None,
        None,
        Some("projectA,projectB"),
        None,
        None,
        "content +projectA +projectB"
    )]
    #[test_case(
        "content",
        None,
        None,
        Some("projectA,projectB"),
        Some("contextA,contextB"),
        None,
        "content +projectA +projectB @contextA @contextB"
    )]
    #[test_case(
        "content",
        Some("2000-1-1"),
        Some("A"),
        Some("projectA,projectB"),
        Some("contextA,contextB"),
        None,
        "(A) 2000-1-1 content +projectA +projectB @contextA @contextB"
    )]
    #[test_case(
        "content",
        Some("2000-1-1"),
        Some("A"),
        Some("projectA,projectB"),
        Some("contextA,contextB"),
        Some("2000-1-2"),
        "x (A) 2000-1-2 2000-1-1 content +projectA +projectB @contextA @contextB"
    )]
    fn to_formatted_string(
        content: &str,
        creation_date: Option<&str>,
        priority: Option<&str>,
        projects: Option<&str>,
        contexts: Option<&str>,
        completion_date: Option<&str>,
        res: &str,
    ) {
        let mut todo = super::Todo::new(content, creation_date, priority, projects, contexts);
        if let Some(completion_date) = completion_date {
            todo.complete(completion_date.to_string());
        }
        assert_eq!(super::Converter::to_formatted_string(&todo), res)
    }

    #[test_case("content")]
    #[test_case("x todo text")]
    #[test_case("(A) todo text")]
    #[test_case("x (A) todo text")]
    #[test_case("(A) 2000-1-1 todo text")]
    #[test_case("x (A) 2000-1-1 todo text")]
    #[test_case("x (A) 2000-1-1 1999-12-31 todo text")]
    #[test_case("todo text +projectA +projectB")]
    #[test_case("todo text +projectA +projectB @contextA @contextB")]
    fn from_formatted_string(formatted_string: &str) {
        let todo = super::Converter::from_formatted_string(formatted_string, None);
        assert_eq!(
            super::Converter::to_formatted_string(&todo),
            formatted_string
        )
    }
}
