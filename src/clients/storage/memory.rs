use crate::{clients::storage::StorageClient, todo::Todo, user::User};

pub struct MemoryStorageClient {
    todos: Vec<Todo>,
    users: Vec<User>,
}

impl StorageClient for MemoryStorageClient {
    fn new() -> Self {
        Self {
            todos: Vec::new(),
            users: Vec::new(),
        }
    }

    fn todo_create(&mut self, todo: Todo) -> Result<(), ()> {
        self.todos.push(todo);

        Ok(())
    }

    fn todo_list(&self) -> Result<Vec<Todo>, ()> {
        Ok(self.todos.clone())
    }

    fn todo_len(&self) -> usize {
        self.todos.len()
    }

    fn todo_dump(&mut self) -> Result<(), ()> {
        self.todos = Vec::new();

        Ok(())
    }

    fn user_create(&mut self, user: User) -> Result<(), ()> {
        self.users.push(user);

        Ok(())
    }

    fn user_len(&self) -> usize {
        self.users.len()
    }
}
