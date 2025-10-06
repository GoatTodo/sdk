/*
    Goat Todo SDK

    Potential API Brainstorming

    - GoatTodoClient -

    let client = Client::new();
    client.login(user)
    client.logout()

    client.add_todo
    client.get_todo
    client.update_todo
    client.delete_todo

    - Todo -

    ?

    The client represents the storage abstraction for todo's.

    let client = Client::new(cloud | filesystem | memory | local db);
    client.login(user);
    client.logout(user);

    User -> email & password


    Client -> Users -> login, logout
    Client -> Todos -> crud
    Client -> Automations -> Generator

    generator(todo templates, day/time/auto rules);


    client.todos.get()
*/

use crate::clients::{
    storage::{StorageClient, StorageType},
    todos::TodosClient,
    users::UsersClient,
};

pub struct Client {
    storage_client: StorageClient,
    users_client: Option<UsersClient>,
    //todos_client: TodosClient,
}

impl Client {
    pub fn new(storage_type: StorageType) -> Self {
        Self {
            storage_client: StorageClient::new(storage_type),
            users_client: None,
        }
    }

    pub fn login(&self) {
        let users_client = UsersClient::new(&self);
        self.users_client = Some(users_client);
    }

    /// let c = Client::new();
    /// c.users().add()
    pub fn users(&self) -> &UsersClient {
        &self.users_client
    }
}
