use std::{cell::RefCell, rc::Rc};

use crate::clients::{storage::StorageClient, todo::TodoClient, user::UserClient};

pub struct Client<T: StorageClient> {
    storage_client: Rc<RefCell<T>>,
    todo_client: TodoClient<T>,
    user_client: UserClient,
}

impl<T: StorageClient> Client<T> {
    pub fn new() -> Self {
        let storage_client: Rc<RefCell<T>> = Rc::new(RefCell::new(T::new()));

        Self {
            storage_client: Rc::clone(&storage_client),
            todo_client: TodoClient::new(Rc::clone(&storage_client)),
            user_client: UserClient::new(),
        }
    }

    pub fn todos(&mut self) -> &mut TodoClient<T> {
        &mut self.todo_client
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{clients::storage::memory::MemoryStorageClient, timestamp::Timestamp, todo::Todo};

    #[test]
    fn test() {
        let mut c = Client::<MemoryStorageClient>::new();

        // TODO: make timestamp not optional so new always works
        let t = Todo::new(
            String::from("Todo Title"),
            Some(String::from("Todo Description")),
            false,
            None,
            None,
            None,
        );

        let r = c.todos().add(t.expect("timestamp should be valid"));
        assert_eq!(Ok(()), r);
    }
}
