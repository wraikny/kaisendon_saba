// use super::super::{
//     error::{ Error },
// };

use super::{
    user::{UserID, User, WaitingUsers},
    setting::{Setting},
};

use super::super::{
    json::{
        self,
        LoginInfo,
    },
    game::{
        RoomID,
        room::{Room},
    },
    error::{ErrorKind, Error}
};

use std::collections::{
    HashMap,
};

#[derive(Debug)]
crate struct Model {
    crate next_user_id : UserID,
    crate users : HashMap<UserID, User>,
    crate waitings : WaitingUsers,

    crate next_room_id : RoomID,
    crate rooms : HashMap<RoomID, Room>,

    crate setting : Setting,
}

impl Model {
    crate fn new(setting : Setting) -> Model {
        Model {
            next_user_id : 0,
            users : HashMap::new(),
            waitings : WaitingUsers::new(),

            next_room_id : 0,
            rooms : HashMap::new(),

            setting : setting,
        }
    }

    crate fn user(&self, id: &UserID) -> Option<User> {
        self.users.get(id).cloned()
    }

    crate fn user_mut(&mut self, id: &UserID) -> Option<&mut User> {
        self.users.get_mut(id)
    }

    pub fn user_json(&self, id: &UserID) -> Option<json::User> {
        let user = self.user(id)?;
        Some(json::User {
            id: user.id,
            name: user.name,
        })
    }

    fn create_user(&mut self, info: &LoginInfo) -> UserID {
        let id = self.next_user_id;
        self.next_user_id += 1;

        let user = User::new(id, &info.username);
        self.users.insert(id, user);

        id
    }

    fn create_room(&mut self, user1: UserID, user2: UserID) -> RoomID {
        let id = self.next_room_id;
        self.next_room_id += 1;

        let room = Room::new(id, &user1, &user2);
        self.rooms.insert(id, room);

        self.user_mut(&user1).unwrap().set_room(id);
        self.user_mut(&user2).unwrap().set_room(id);

        id
    }

    crate fn room(&self, id: &RoomID) -> Option<Room> {
        self.rooms.get(id).cloned()
    }

    crate fn room_mut(&mut self, id: &RoomID) -> Option<&mut Room> {
        self.rooms.get_mut(id)
    }

    crate fn room_json(&self, id: &RoomID) -> Option<json::Room> {
        let room = self.room(id)?;
        let user1 = self.user_json(&room.user1.id)?;
        
        let user2 = self.user_json(&room.user2.id)?;

        Some(json::Room {
            id: room.id,
            user1: user1,
            user2: user2,
        })
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
                let remove_ids =
                    self.rooms.iter()
                        .filter(|x| x.1.contains(&id))
                        .map(|x| x.0.clone())
                        .collect::<Vec<_>>()
                        .into_iter();
                
                for r_id in remove_ids {
                    self.rooms.remove(&r_id);
                }
                
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
        let setting = super::Setting::new((12, 12));
        let mut model = super::Model::new(setting);
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