use crate::Result;
mod kv;
mod sled;
pub trait KvsEngine {
    fn set(&self, key: String, value: String) -> Result<()>;

    fn get(&self, key: String) -> Result<Option<String>>;

    fn remove(&self, key: String) -> Result<()>;
}

pub use self::kv::KvStore;
pub use self::sled::SledKvsEngine;
