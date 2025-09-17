use crate::uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: Option<String>, email: String, password: String) -> Self {
        Self {
            id: Uuid::new(),
            name,
            email,
            password,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uuid::FAKE_UUID;

    #[test]
    fn test_new() {
        let name = Some(String::from("Joe"));
        let email = String::from("joe@goattodo.com");
        let password = String::from("password123");

        let user = User::new(name.clone(), email.clone(), password.clone());

        assert_eq!(FAKE_UUID.to_string(), user.id.get());
        assert_eq!(name, user.name);
        assert_eq!(email, user.email);
        assert_eq!(password, user.password);
    }
}
