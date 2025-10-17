use crate::{clients::storage::StorageClient, todo::Todo};

pub struct MemoryStorageClient {
    todos: Vec<Todo>,
}

impl StorageClient for MemoryStorageClient {
    fn new() -> Self {
        Self { todos: Vec::new() }
    }

    fn todo_create(&mut self, todo: Todo) -> Result<(), ()> {
        self.todos.push(todo);

        Ok(())
    }

    fn todo_list(&self) -> Result<Vec<Todo>, ()> {
        Ok(self.todos.clone())
    }

    fn todo_dump(&mut self) -> Result<(), ()> {
        self.todos = Vec::new();

        Ok(())
    }
}
