use futures_util::lock::{Mutex, MutexGuard};
use std::thread::sleep;

pub trait GetMutex<T: ?Sized> {
    fn get_mutex(&self) -> MutexGuard<'_, T>;
}

impl<T: ?Sized> GetMutex<T> for Mutex<T> {
    fn get_mutex(&self) -> MutexGuard<'_, T> {
        loop {
            match self.try_lock() {
                Some(guard) => return guard,
                None => sleep(std::time::Duration::from_millis(10)),
            }
        }
    }
}
