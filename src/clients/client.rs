use crate::clients::{todo::TodoClient, user::UserClient};

pub struct Client {
    todo_client: TodoClient,
    user_client: UserClient,
}

impl Client {
    pub fn new() -> Self {
        Self {
            todo_client: TodoClient::new(),
            user_client: UserClient::new(),
        }
    }

    pub fn todos(&mut self) -> &mut TodoClient {
        &mut self.todo_client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut c = Client::new();

        c.todos().add();
        assert_eq!(String::from("a"), c.todos().get());

        c.todos().add();
        assert_eq!(String::from("aa"), c.todos().get());
    }
}
