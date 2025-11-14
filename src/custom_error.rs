pub type Result<T> = std::result::Result<T, CustomError>;

#[derive(Clone, Debug, PartialEq)]
pub enum CustomError {
    UserNameIsSomeAndEmpty,
    UserEmailIsEmpty,
    UserPasswordIsEmpty,
}
