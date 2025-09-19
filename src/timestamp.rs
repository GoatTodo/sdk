use std::time::SystemTime;

#[derive(Copy, Clone)]
pub struct Timestamp {
    data: u64,
}

impl Timestamp {
    /// Creates a Timestamp for the current system UTC time.
    ///
    /// Note: This uses std::time::SystemTime, which is currently not monotonic
    /// according to the docs.
    pub fn now() -> Result<Self, String> {
        if let Ok(dur) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(Self {
                data: dur.as_secs(),
            })
        } else {
            Err(String::from(
                "Error: Couldn't create a Timestamp, the current time is before the Unix epoch.",
            ))
        }
    }

    pub fn get_current_timestamp(&self) -> u64 {
        self.data
    }
}

impl Default for Timestamp {
    /// Creates a Timestamp with an internal data value of 0, representing the
    /// Unix epoch: January 1, 1970 UTC.
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        const UNIX_EPOCH: u64 = 0;
        let current_timestamp = Timestamp::now().unwrap();

        assert_eq!(true, current_timestamp.data > UNIX_EPOCH);
    }

    #[test]
    fn test_default() {
        const UNIX_EPOCH: u64 = 0;
        let current_timestamp = Timestamp::default();

        assert_eq!(UNIX_EPOCH, current_timestamp.data);
    }
}
