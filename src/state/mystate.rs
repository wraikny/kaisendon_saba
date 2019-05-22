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

#[derive(Debug, Clone)]
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

    pub fn print_model(&self) {
        println!("{:?}", self.model());
    }

    pub fn add_newuser(&self, info : &LoginInfo) -> Result<UserID, Error> {
        let mut model = self.model()?;
        let id = model.add_newuser(info);
        Ok(id)
    }

    pub fn remove_user(&self, id: &UserID) -> Result<bool, Error> {
        let mut model = self.model()?;
        Ok(model.remove_user(id))
    }
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::sync::mpsc;

    fn init_state() -> super::MyState {
        let state = super::MyState::new();

        let (tx, rx) = mpsc::channel();

        for i in 0..10 {
            let (state, tx) = (state.clone(), tx.clone());
            thread::spawn(move || {
                state.add_newuser(&super::LoginInfo{ username: format!("user_{}", i) });
                tx.send(()).unwrap();
            });
        }

        for _ in 0..10 {
            rx.recv().unwrap();
        }

        state
    }
    
    #[test]
    fn add_newuser() {
        let state = init_state();

        dbg!(state);
    }

    #[test]
    fn remove_user() {
        let state = init_state();

        {
            let (tx, rx) = mpsc::channel();

            for i in 0..10 {
                let (state, tx) = (state.clone(), tx.clone());
                thread::spawn(move || {
                    state.remove_user(&i);
                    tx.send(()).unwrap();
                });
            }

            for _ in 0..10 {
                rx.recv().unwrap();
            }
        }

        dbg!(state);
    }
}