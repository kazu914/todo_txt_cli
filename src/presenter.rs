use super::model::Todo;
use cli_table::{print_stdout, Cell, Style, Table};
pub struct Presenter {
    todos: Vec<Todo>,
}

impl Presenter {
    pub fn new(todos: Vec<Todo>) -> Presenter {
        Presenter { todos }
    }

    pub fn print(&self) {
        for (i, todo) in self.todos.iter().enumerate() {
            println!("{}: {}", i, todo.to_formatted_string());
        }
    }

    pub fn pring_table(&self) {
        let table_formats: Vec<Vec<String>> =
            self.todos.iter().map(Presenter::to_table_format).collect();

        let table = table_formats
            .table()
            .title(vec![
                "key".cell().bold(true),
                "completed?".cell().bold(true),
                "priority".cell().bold(true),
                "completion date".cell().bold(true),
                "creation date".cell().bold(true),
                "projects".cell().bold(true),
                "contexts".cell().bold(true),
                "content".cell().bold(true),
            ])
            .bold(true);
        let _ = print_stdout(table);
    }

    fn to_table_format(todo: &Todo) -> Vec<String> {
        let key = if todo.key().is_some() {
            todo.key().unwrap().to_string()
        } else {
            "-".to_string()
        };
        let is_copleted_string = if *todo.is_completed() {
            "x".to_string()
        } else {
            "-".to_string()
        };

        let priority_string = todo.priority().clone().unwrap_or_else(|| "-".to_string());
        let completion_date_string = todo
            .completion_date()
            .clone()
            .unwrap_or_else(|| "-".to_string());
        let creation_date_string = todo
            .creation_date()
            .clone()
            .unwrap_or_else(|| "-".to_string());

        let projects_string = todo
            .projects()
            .clone()
            .unwrap_or_else(|| vec!["-".to_string()])
            .join(" ");
        let contexts_string = todo
            .contexts()
            .clone()
            .unwrap_or_else(|| vec!["-".to_string()])
            .join(" ");

        let content_string = todo.content().clone();

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
    use super::{Presenter, Todo};
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
            assert_eq!(super::Presenter::to_table_format(&todo), table_format);
        }
    }
}
