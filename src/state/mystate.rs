use std::sync::{Arc, Mutex, LockResult, MutexGuard};
use super::model::{Model};

pub struct MyState {
    model : Arc<Mutex<Model>>
}

impl MyState {
    pub fn new(model : Model) -> MyState {
        MyState {
            model : Arc::new(Mutex::new(model))
        }
    }

    pub fn model(&self) -> LockResult<MutexGuard<'_, Model>> {
        self.model.lock()
    }
}