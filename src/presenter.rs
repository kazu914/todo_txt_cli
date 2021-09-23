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
        let table_formats: Vec<Vec<String>> = self
            .todos
            .iter()
            .map(|todo| todo.to_table_format())
            .collect();

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
}
