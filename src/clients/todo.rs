use std::{cell::RefCell, rc::Rc};

use crate::{clients::storage::StorageClient, todo::Todo};

pub struct TodoClient<T: StorageClient> {
    storage_client: Rc<RefCell<T>>,
}

impl<T: StorageClient> TodoClient<T> {
    pub fn new(storage_client: Rc<RefCell<T>>) -> Self {
        Self { storage_client }
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
