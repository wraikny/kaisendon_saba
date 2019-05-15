use std::sync::{Arc, Mutex, MutexGuard};
use super::{
    model::Model,
    error::{ Error, MyPoisonError },
};

#[derive(Debug)]
pub struct MyState {
    log : Arc<Mutex<()>>,
    model : Arc<Mutex<Model>>,
}

impl MyState {
    fn new_mutex<T>(item : T) -> Arc<Mutex<T>> {
        Arc::new(Mutex::new(item))
    }

    pub fn new() -> Self {
        MyState {
            log : Self::new_mutex(()),
            model : Self::new_mutex(Model::new()),
        }
    }

    fn model(&self) -> Result<MutexGuard<'_, Model>, Error> {
        self.model.lock()
            .map_err(MyPoisonError::from)
            .map_err(Error::from)
    }

    pub fn add_newuser(&self) -> Result<u32, Error> {
        let mut model = self.model()?;
        let id = model.add_newuser();
        Ok(id)
    }
}