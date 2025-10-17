pub struct UserClient {
    email: Option<String>,
    password: Option<String>,
}

impl UserClient {
    pub fn new() -> Self {
        Self {
            email: None,
            password: None,
        }
    }

    pub fn login(&mut self, email: String, password: String) -> bool {
        self.email = Some(email);
        self.password = Some(password);

        true
    }

    pub fn logout(&mut self) {
        self.email = None;
        self.password = None;
    }
}
