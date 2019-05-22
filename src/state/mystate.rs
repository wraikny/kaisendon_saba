use std::sync::{Arc, Mutex, MutexGuard};

use super::super::{
    error::{ Error, MyPoisonError },
};

use super::{
    model::Model,
    user::{UserID, User},
};

use super::super::json::{
    login::LoginInfo,
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

    pub fn model_string(&self) -> String {
        format!("{:?}", self.model())
    }

    pub fn add_newuser(&self, info : &LoginInfo) -> Result<User, Error> {
        let mut model = self.model()?;
        let user = model.add_newuser(info);
        Ok(user)
    }
}