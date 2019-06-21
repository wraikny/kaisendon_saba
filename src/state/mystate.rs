use std::sync::{Arc, Mutex, MutexGuard};

use crate::{
    error::{Error, ErrorKind, MyPoisonError},
    json::{
        self,
        game::{Attack, Ships},
        LoginInfo, // UserInfo,
    },
    // game::{
    //     RoomID,
    // },
    state::{
        model::Model,
        setting::Setting,
        user::{User, UserID},
    },

};

#[derive(Debug, Clone)]
pub struct MyState {
    log: Arc<Mutex<()>>,
    model: Arc<Mutex<Model>>,
}

impl MyState {
    fn new_mutex<T>(item: T) -> Arc<Mutex<T>> {
        Arc::new(Mutex::new(item))
    }

    pub fn new(setting: Setting) -> Self {
        MyState {
            log: Self::new_mutex(()),
            model: Self::new_mutex(Model::new(setting)),
        }
    }

    fn model(&self) -> Result<MutexGuard<'_, Model>, Error> {
        self.model
            .lock()
            .map_err(MyPoisonError::from)
            .map_err(Error::from)
    }

    pub fn print_model(&self) {
        println!("{:?}", self.model());
    }

    // fn user(&self, id: &UserID) -> Result<User, Error> {
    //     self.model()?.user(id)
    //         .ok_or(ErrorKind::Optional.into())
    // }

    crate fn get_user_json(&self, id: &UserID) -> Result<json::User, Error> {
        let model = self.model()?;
        model
            .user_json(id)
            .ok_or("User is not found".to_owned())
            .map_err(ErrorKind::String)
            .map_err(Error::from)
    }

    crate fn user_pop_jsons(&self, id: &UserID) -> Result<Vec<String>, Error> {
        let mut model = self.model()?;
        model
            .user_pop_jsons(id)
            .map_err(ErrorKind::String)
            .map_err(Error::from)
    }

    crate fn add_newuser(&self, info: &LoginInfo) -> Result<UserID, Error> {
        let mut model = self.model()?;
        let id = model.add_newuser(info);
        Ok(id)
    }

    crate fn remove_user(&self, id: &UserID) -> Result<bool, Error> {
        let mut model = self.model()?;
        Ok(model.remove_user(id))
    }

    crate fn check_waiting(&self, id: &UserID) -> Result<json::UserWaitingState, Error> {
        let model = self.model()?;
        let user = (model.user(id).ok_or(ErrorKind::Optional.into()) as Result<User, Error>)?;
        match user.room_id {
            Some(room_id) => {
                let room = (model.room_json(&room_id).ok_or(ErrorKind::Optional.into())
                    as Result<json::Room, Error>)?;
                Ok(json::UserWaitingState::InRoom(room))
            }
            None => Ok(json::UserWaitingState::Waiting),
        }
    }

    crate fn add_ships(&self, ships: &Ships) -> Result<(), Error> {
        let mut model = self.model()?;
        model
            .add_ships(&ships.id, &ships.ships)
            .map_err(ErrorKind::String)
            .map_err(Error::from)
    }

    crate fn attack(&self, attack: Attack) -> Result<(), Error> {
        let mut model = self.model()?;
        model
            .attack(attack)
            .map_err(ErrorKind::String)
            .map_err(Error::from)
    }

}

#[cfg(test)]
mod test {

    use std::sync::mpsc;
    use std::thread;
    fn init_state() -> super::MyState {
        let setting = super::Setting::new((12, 12));
        let state = super::MyState::new(setting);

        let (tx, rx) = mpsc::channel();

        for i in 0..10 {
            let (state, tx) = (state.clone(), tx.clone());
            thread::spawn(move || {
                state
                    .add_newuser(&super::LoginInfo {
                        username: format!("user_{}", i),
                    })
                    .unwrap();
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
                    state.remove_user(&i).unwrap();
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