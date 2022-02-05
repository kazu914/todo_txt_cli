use super::model::Todo;
pub fn filter(todos: Vec<Todo>, show_completed: bool) -> Vec<Todo> {
    if show_completed {
        todos
    } else {
        todos
            .into_iter()
            .filter(|todo| !*todo.is_completed())
            .collect()
    }
}
