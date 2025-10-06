pub enum StorageType {
    Filesystem,
}

pub struct StorageClient {
    storage_type: StorageType,
}

impl StorageClient {
    pub fn new(storage_type: StorageType) -> Self {
        Self { storage_type }
    }
}
