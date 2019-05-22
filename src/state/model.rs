use super::super::{
    error::{ Error },
};

use super::{
    user::{UserID, User, WaitingUsers},
    room::{Room, RoomID},
};

use super::super::json::{
    login::LoginInfo,
};

use std::collections::{
    HashMap,
};

use std::{
    sync::{Arc, Mutex}
};

#[derive(Debug)]
crate struct Model {
    crate next_user_id : UserID,
    crate users : HashMap<UserID, Arc<Mutex<User>>>,
    crate waitings : WaitingUsers,

    crate next_room_id : RoomID,
    crate rooms : HashMap<RoomID, Arc<Mutex<Room>>>,
}

impl Model {
    crate fn new() -> Model {
        Model {
            next_user_id : 0,
            users : HashMap::new(),
            waitings : WaitingUsers::new(),

            next_room_id : 0,
            rooms : HashMap::new(),
        }
    }

    crate fn user(&self, id: &UserID) -> Option<&Arc<Mutex<User>>> {
        self.users.get(id)
    }

    fn create_user(&mut self, info: &LoginInfo) -> UserID {
        let id = self.next_user_id;
        self.next_user_id += 1;

        let user = Arc::new(Mutex::new(User::new(id, &info.username)));
        self.users.insert(id, user);

        id
    }

    fn create_room(&mut self, user1: UserID, user2: UserID) -> RoomID {
        let id = self.next_room_id;
        self.next_room_id += 1;

        let room = Arc::new(Mutex::new(Room::new(id, user1, user2)));
        self.rooms.insert(id, room);

        self.user(&user1).unwrap().lock().unwrap().set_room(id);
        self.user(&user2).unwrap().lock().unwrap().set_room(id);

        id
    }

    crate fn room(&self, id: &RoomID) -> Option<&Arc<Mutex<Room>>> {
        self.rooms.get(id)
    }

    crate fn add_newuser(&mut self, info : &LoginInfo) -> UserID {
        let id = self.create_user(info);

        let waitings : &mut Vec<_> = self.waitings.larger_mut();
        waitings.push(id);

        if waitings.len() == 2 {
            let user1 = waitings.pop().unwrap();
            let user2 = waitings.pop().unwrap();

            let _ = self.create_room(user1, user2);
        }

        id
    }

    crate fn remove_user(&mut self, id: &UserID) -> bool {
        match self.users.remove(id) {
            Some(_) => {
                self.rooms = self.rooms.clone().into_iter().filter_map(|(k, v)|{
                    if v.lock().unwrap().contains(id) {
                        None
                    } else {
                        Some( (k, v) )
                    }
                }).collect();
                self.waitings.remove_user(id);
                true
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn add_remove_user() {
        let mut model = super::Model::new();
        let mut ids = Vec::new();

        for i in 0..10 {
            let id = model.add_newuser(&super::LoginInfo{ username: format!("hoge_{}", i) });
            ids.push(id);
        }

        for id in ids.into_iter() {
            model.remove_user(&id);
        }

        // let newModel = super::Model::new();
        
        assert_eq!(model.next_user_id, 10);
        assert_eq!(model.users.len(), 0);
        assert_eq!(model.next_room_id, 5);
        assert_eq!(model.rooms.len(), 0);
        assert_eq!(model.waitings.larger().len(), 0);
    }
}