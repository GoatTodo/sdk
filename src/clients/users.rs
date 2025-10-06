use crate::clients::client::Client;

pub(super) struct UsersClient<'a> {
    client: &'a Client,
}

impl<'a> UsersClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create() -> String {
        String::from("we return a user")
    }
}
