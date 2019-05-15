use std::sync::{Arc, Mutex, LockResult, MutexGuard};
use super::{
    model::Model,
    room::Room,
    user::User,
    error::{ Error, MyPoisonError },
};

// use failure::{SyncFailure};

#[derive(Debug)]
struct WaitingUsers {
    winners : Vec<Box<User>>,
    losers : Vec<Box<User>>,
}

impl WaitingUsers {
    fn new() -> Self {
        WaitingUsers {
            winners : Vec::new(),
            losers : Vec::new(),
        }
    }

    fn get_longer(&mut self) -> &mut Vec<Box<User>> {
        if self.losers.len() >= self.winners.len() {
            &mut self.losers
        } else {
            &mut self.winners
        }
    }

    fn get_shorter(&mut self) -> &mut Vec<Box<User>> {
        if self.losers.len() < self.winners.len() {
            &mut self.losers
        } else {
            &mut self.winners
        }
    }
}

#[derive(Debug)]
pub struct MyState {
    log : Arc<Mutex<()>>,
    model : Arc<Mutex<Model>>,
    rooms : Arc<Mutex<Vec<Room>>>,
    waitings : Arc<Mutex<WaitingUsers>>
}

impl MyState {
    fn new_field<T>(item : T) -> Arc<Mutex<T>> {
        Arc::new(Mutex::new(item))
    }

    pub fn new() -> Self {
        MyState {
            log : Self::new_field(()),
            model : Self::new_field(Model::new()),
            rooms : Self::new_field(Vec::new()),
            waitings : Self::new_field(WaitingUsers::new()),
        }
    }

    pub fn log(&self) -> LockResult<MutexGuard<'_, ()>> {
        self.log.lock()
    }

    fn model(&self) -> LockResult<MutexGuard<'_, Model>> {
        self.model.lock()
    }

    fn rooms(&self) -> LockResult<MutexGuard<'_, Vec<Room>>> {
        self.rooms.lock()
    }

    fn waitings(&self) -> LockResult<MutexGuard<'_, WaitingUsers>> {
        self.waitings.lock()
    }

    fn add_user<T>(&self) -> Result<(), Error> {
        let mut model : MutexGuard<'_, Model> = self.model()
            .map_err(MyPoisonError::from)
            .map_err(Error::from)?;

        let user = User { id : model.next_id };
        model.next_id += 1;
        let mut waitings : MutexGuard<'_, WaitingUsers> = self.waitings()
            .map_err(MyPoisonError::from)
            .map_err(Error::from)?;
        

        let user = Box::new(user);
        waitings.get_shorter().push(user);
        Ok(())
    }
}