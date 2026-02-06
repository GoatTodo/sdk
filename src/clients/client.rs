use std::{cell::RefCell, rc::Rc};

use crate::clients::{
    auth_state::AuthState, storage::StorageClient, todo::TodoClient, user::UserClient,
};

pub struct Client<T: StorageClient> {
    // Internally owned sub-clients
    todo_client: TodoClient<T>,
    user_client: UserClient<T>,

    // Internal client shared among the sub-clients
    storage_client: Rc<RefCell<T>>,

    // Internal state shared among the sub-clients
    auth_state: Rc<RefCell<AuthState>>,
}

impl<T: StorageClient> Client<T> {
    pub fn new() -> Self {
        let auth_state: Rc<RefCell<AuthState>> = Rc::new(RefCell::new(AuthState::new()));
        let storage_client: Rc<RefCell<T>> = Rc::new(RefCell::new(T::new()));

        Self {
            todo_client: TodoClient::new(Rc::clone(&storage_client), Rc::clone(&auth_state)),
            user_client: UserClient::new(Rc::clone(&storage_client), Rc::clone(&auth_state)),
            storage_client: Rc::clone(&storage_client),
            auth_state: Rc::clone(&auth_state),
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

    #[test]
    fn delete_user_success() {
        // Create the client
        let mut c = Client::<MemoryStorageClient>::new();

        // Create the user
        let user = User::new(
            Some(String::from("John")),
            String::from("john@example.com"),
            String::from("correct_horse_battery_staple"),
        )
        .expect("user created successfully");

        // Add the user
        let user_id = user.id();
        let _user_creation_result = c.users().create(user);

        // Delete the user
        let deletion_result = c.users().delete(user_id);

        // Verify
        const EXPECTED_USER_COUNT: usize = 0;
        assert_eq!(Ok(()), deletion_result);

        assert_eq!(
            EXPECTED_USER_COUNT,
            c.users().len().expect("no internal errors")
        );
    }

    #[test]
    fn login_user_success() {
        // Create the client
        let mut c = Client::<MemoryStorageClient>::new();

        // Create the user
        let user_name = String::from("John)");
        let user_email = String::from("john@example.com");
        let user_password = String::from("correct_horse_battery_staple");
        let user = User::new(Some(user_name), user_email.clone(), user_password.clone())
            .expect("user created successfully");

        // Add the user to the client
        let _ = c.users().create(user.clone());

        // Log in the user
        let logged_in_user = c
            .users()
            .login(user_email, user_password)
            .expect("user logged in");

        // Verify
        assert!(c.auth_state.borrow().logged_in_user_id.is_some());
        assert_eq!(user.id(), logged_in_user.id());
    }
}
