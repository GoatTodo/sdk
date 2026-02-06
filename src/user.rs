use uuid::Uuid;

use crate::custom_error::{CustomError, Result};

#[derive(Clone, Debug)]
pub struct User {
    uuid: Uuid,
    pub name: Option<String>,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: Option<String>, email: String, password: String) -> Result<Self> {
        if let Some(name) = &name {
            if name.trim().len() == 0 {
                return Err(CustomError::UserNameIsSomeAndEmpty);
            }
        }

        if email.trim().len() == 0 {
            return Err(CustomError::UserEmailIsEmpty);
        }

        if password.trim().len() == 0 {
            return Err(CustomError::UserPasswordIsEmpty);
        }

        Ok(Self {
            uuid: Uuid::new_v4(),
            name,
            email,
            password,
        })
    }

    pub fn id(&self) -> Uuid {
        self.uuid.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants;

    #[test]
    fn test_new_success() {
        let name = Some(String::from("Joe"));
        let email = String::from("joe@example.com");
        let password = String::from("password123");

        let user =
            User::new(name.clone(), email.clone(), password.clone()).expect("valid user created");

        assert_eq!(name, user.name);
        assert_eq!(email, user.email);
        assert_eq!(password, user.password);
    }

    #[test]
    fn test_new_name_empty_failure() {
        let name = Some(String::from(""));
        let email = String::from("joe@example.com");
        let password = String::from("password123");

        let user = User::new(name.clone(), email.clone(), password.clone());

        assert_eq!(CustomError::UserNameIsSomeAndEmpty, user.unwrap_err());
    }

    #[test]
    fn test_new_name_whitespace_failure() {
        let name = Some(String::from(" "));
        let email = String::from("joe@example.com");
        let password = String::from("password123");

        let user = User::new(name.clone(), email.clone(), password.clone());

        assert_eq!(CustomError::UserNameIsSomeAndEmpty, user.unwrap_err());
    }

    #[test]
    fn test_new_email_empty_failure() {
        let name = Some(String::from("Joe"));
        let email = String::from("");
        let password = String::from("password123");

        let user = User::new(name.clone(), email.clone(), password.clone());

        assert_eq!(CustomError::UserEmailIsEmpty, user.unwrap_err());
    }

    #[test]
    fn test_new_email_whitespace_failure() {
        let name = Some(String::from("Joe"));
        let email = String::from(" ");
        let password = String::from("password123");

        let user = User::new(name.clone(), email.clone(), password.clone());

        assert_eq!(CustomError::UserEmailIsEmpty, user.unwrap_err());
    }

    #[test]
    fn test_new_password_empty_failure() {
        let name = Some(String::from("Joe"));
        let email = String::from("joe@example.com");
        let password = String::from("");

        let user = User::new(name.clone(), email.clone(), password.clone());

        assert_eq!(CustomError::UserPasswordIsEmpty, user.unwrap_err());
    }

    #[test]
    fn test_new_password_whitespace_failure() {
        let name = Some(String::from("Joe"));
        let email = String::from("joe@example.com");
        let password = String::from(" ");

        let user = User::new(name.clone(), email.clone(), password.clone());

        assert_eq!(CustomError::UserPasswordIsEmpty, user.unwrap_err());
    }

    #[test]
    fn test_id_success() {
        let name = Some(String::from("Joe"));
        let email = String::from("joe@example.com");
        let password = String::from("password123");

        let user =
            User::new(name.clone(), email.clone(), password.clone()).expect("valid user created");

        assert_eq!(constants::UUID_STRING_LENGTH, user.id().to_string().len());
        assert_eq!(name, user.name);
        assert_eq!(email, user.email);
        assert_eq!(password, user.password);
    }
}
