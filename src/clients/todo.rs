pub struct TodoClient {
    pub data: String,
}

impl TodoClient {
    pub fn new() -> Self {
        Self {
            data: String::from(""),
        }
    }

    pub fn add(&mut self) -> bool {
        self.data.push('a');

        true
    }

    pub fn get(&self) -> String {
        self.data.clone()
    }
}
