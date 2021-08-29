use crate::Result;
mod naive;
pub use self::naive::NaiveThreadPool;
pub trait ThreadPool {
    fn new(size: u32) -> Result<Self>
    where
        Self: Sized;

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
