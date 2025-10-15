use crate::clients::todo::TodoClient;

pub struct Client {
    todos_client: TodoClient,
}

impl Client {
    pub fn new() -> Self {
        Self {
            todos_client: TodoClient::new(),
        }
    }

    pub fn todos(&mut self) -> &mut TodoClient {
        &mut self.todos_client
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
    }
}
