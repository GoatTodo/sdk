pub struct User {
    pub name: Option<String>,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: Option<String>, email: String, password: String) -> Self {
        Self {
            name,
            email,
            password,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let name = Some(String::from("Joe"));
        let email = String::from("joe@goattodo.com");
        let password = String::from("password123");

        let user = User::new(name.clone(), email.clone(), password.clone());

        assert_eq!(name, user.name);
        assert_eq!(email, user.email);
        assert_eq!(password, user.password);
    }
}
