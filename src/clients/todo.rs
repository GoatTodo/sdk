use std::{cell::RefCell, rc::Rc};

use crate::{
    clients::{auth_state::AuthState, storage::StorageClient},
    todo::Todo,
};

pub struct TodoClient<T: StorageClient> {
    storage_client: Rc<RefCell<T>>,
    auth_state: Rc<RefCell<AuthState>>,
}

impl<T: StorageClient> TodoClient<T> {
    pub fn new(storage_client: Rc<RefCell<T>>, auth_state: Rc<RefCell<AuthState>>) -> Self {
        Self {
            storage_client,
            auth_state,
        }
    }

    pub fn add(&mut self, todo: Todo) -> Result<(), ()> {
        let Ok(mut sc) = self.storage_client.try_borrow_mut() else {
            return Err(());
        };

        sc.todo_create(todo)
    }

    pub fn len(&self) -> Result<usize, ()> {
        let Ok(mut sc) = self.storage_client.try_borrow_mut() else {
            return Err(());
        };

        Ok(sc.todo_len())
    }
}
