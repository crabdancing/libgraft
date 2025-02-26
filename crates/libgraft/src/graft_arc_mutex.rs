use std::sync::{Arc, Mutex, MutexGuard};

pub trait GraftArcMutex<T> {
    fn lunwrap(&self) -> MutexGuard<'_, T>;
}

impl<T> GraftArcMutex<T> for Arc<Mutex<T>> {
    fn lunwrap(&self) -> MutexGuard<'_, T> {
        self.lock().unwrap()
    }
}
