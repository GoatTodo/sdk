pub const FAKE_UUID: &str = "1ac54d4d-bbf3-4db2-9bc7-9e3f152e765b";

pub struct Uuid {
    pub data: String,
}

#[allow(clippy::new_without_default)]
impl Uuid {
    pub fn new() -> Self {
        Uuid {
            data: FAKE_UUID.to_string(),
        }
    }

    pub fn get(&self) -> String {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uuid = Uuid::new();
        assert_eq!(FAKE_UUID.to_string(), uuid.data);
    }

    #[test]
    fn test_get() {
        let uuid = Uuid::new();
        assert_eq!(FAKE_UUID.to_string(), uuid.get());
    }
}
