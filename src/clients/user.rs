use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::{clients::storage::StorageClient, user::User};

pub struct UserClient<T: StorageClient> {
    storage_client: Rc<RefCell<T>>,
    current_user_email: Option<String>,
}

impl<T: StorageClient> UserClient<T> {
    pub fn new(storage_client: Rc<RefCell<T>>) -> Self {
        Self {
            storage_client,
            current_user_email: None,
        }
    }

    pub fn create(&mut self, user: User) -> Result<(), ()> {
        let Ok(mut sc) = self.storage_client.try_borrow_mut() else {
            return Err(());
        };

        sc.user_create(user)
    }

    pub fn delete(&mut self, user_id: Uuid) -> Result<(), ()> {
        let Ok(mut sc) = self.storage_client.try_borrow_mut() else {
            return Err(());
        };

        sc.user_delete(user_id)
    }

    pub fn login(&mut self, email: String, _password: String) -> Result<(), String> {
        if self.current_user_email.is_some() {
            return Err(String::from("A user is already logged in."));
        }

        self.current_user_email = Some(email);

        // TODO: check the storage client to see if the password hash matches correctly

        Ok(())
    }

    pub fn logout(&mut self) {
        self.current_user_email = None;
    }

    pub fn len(&self) -> Result<usize, ()> {
        let Ok(mut sc) = self.storage_client.try_borrow_mut() else {
            return Err(());
        };

        Ok(sc.user_len())
    }
}
