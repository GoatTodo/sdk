use uuid::Uuid;

pub struct AuthState {
    pub logged_in_user_id: Option<Uuid>,
}

impl AuthState {
    pub fn new() -> Self {
        AuthState {
            logged_in_user_id: None,
        }
    }
}
