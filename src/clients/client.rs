use std::{cell::RefCell, rc::Rc};

use crate::clients::{storage::StorageClient, todo::TodoClient, user::UserClient};

pub struct Client<T: StorageClient> {
    storage_client: Rc<RefCell<T>>,
    todo_client: TodoClient<T>,
    user_client: UserClient<T>,
}

impl<T: StorageClient> Client<T> {
    pub fn new() -> Self {
        let storage_client: Rc<RefCell<T>> = Rc::new(RefCell::new(T::new()));

        Self {
            storage_client: Rc::clone(&storage_client),
            todo_client: TodoClient::new(Rc::clone(&storage_client)),
            user_client: UserClient::new(Rc::clone(&storage_client)),
        }
    }

    pub fn todos(&mut self) -> &mut TodoClient<T> {
        &mut self.todo_client
    }

    pub fn users(&mut self) -> &mut UserClient<T> {
        &mut self.user_client
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{clients::storage::memory::MemoryStorageClient, todo::Todo, user::User};

    #[test]
    fn create_todo_success() {
        // Create the client
        let mut c = Client::<MemoryStorageClient>::new();

        // Create the todo
        let todo = Todo::new(
            String::from("Todo Title"),
            Some(String::from("Todo Description")),
            false,
            None,
            None,
            None,
        );
        let todo = todo.expect("timestamp should be valid");

        // Add the todo
        let result = c.todos().add(todo);

        // Verify
        const EXPECTED_TODO_COUNT: usize = 1;
        assert_eq!(Ok(()), result);
        assert_eq!(
            EXPECTED_TODO_COUNT,
            c.todos().len().expect("no internal errors")
        );
    }

    #[test]
    fn create_user_success() {
        let mut c = Client::<MemoryStorageClient>::new();

        let user = User::new(
            Some(String::from("John")),
            String::from("john@example.com"),
            String::from("correct_horse_battery_staple"),
        )
        .expect("user created successfully");

        let u = c.users().create(user);

        assert!(u.is_ok());
    }
}
