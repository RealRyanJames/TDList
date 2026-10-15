#[derive(Default)]

pub struct EmptyTodos {
    todos: Vec<String>,
}

pub fn todos(todo: Vec<String>) -> Vec<String> {
    if todo.len() == 0 {
        let t = EmptyTodos::default();
        t.todos;
    }

    todo
}
