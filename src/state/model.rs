use crate::{
    game::{
        room::{Room, UserKind},
        ship::Ship,
        RoomID,
    },
    json::{
        self,
        game::{Attack, GameFinish},
        LoginInfo,
    },

    // error::{
    //     ErrorKind,
    //     Error,
    // },
    state::{
        setting::Setting,
        user::{User, UserID, WaitingUsers},
    },
};

use serde::Serialize;

extern crate serde_json;

use std::collections::HashMap;

#[derive(Debug)]
crate struct Model {
    crate next_user_id: UserID,
    crate users: HashMap<UserID, User>,
    crate waitings: WaitingUsers,

    crate next_room_id: RoomID,
    crate rooms: HashMap<RoomID, Room>,

    crate setting: Setting,
}

impl Model {
    crate fn new(setting: Setting) -> Model {
        Model {
            next_user_id: 0,
            users: HashMap::new(),
            waitings: WaitingUsers::new(),

            next_room_id: 0,
            rooms: HashMap::new(),

            setting: setting,
        }
    }

    crate fn user(&self, id: &UserID) -> Option<User> {
        self.users.get(id).cloned()
    }

    crate fn user_mut(&mut self, id: &UserID) -> Option<&mut User> {
        self.users.get_mut(id)
    }

    crate fn user_push_json<T: Serialize>(&mut self, id: &UserID, obj: &T) -> Result<(), String> {
        let user = self
            .user_mut(id)
            .ok_or(format!("User({}) is not found", id))?;

        user.push_json(obj)
    }

    crate fn user_pop_jsons(&mut self, id: &UserID) -> Result<Vec<String>, String> {
        let user = self
            .user_mut(id)
            .ok_or(format!("User({}) is not found", id))?;

        Ok(user.pop_jsons())
    }

    crate fn user_json(&self, id: &UserID) -> Option<json::User> {
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

    fn room_mut(&mut self, id: &RoomID) -> Option<&mut Room> {
        self.rooms.get_mut(id)
    }

    fn room_id_of_user(&self, id: &UserID) -> Result<RoomID, String> {
        let user = self.user(id).ok_or("UserID is not found")?;
        user.room_id.ok_or("User is not in room".to_owned())
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

    fn check_new_room(&mut self) -> Option<RoomID> {
        self.waitings
            .pop_pair()
            .map(|(a, b)| self.create_room(a, b))
    }

    fn remove_room(&mut self, id: &RoomID) -> bool {
        match self.rooms.remove(id) {
            Some(room) => {
                let (id1, id2) = room.user_ids();
                self.waitings.push_user(id1);
                self.waitings.push_user(id2);

                let _ = self.check_new_room();
                let _ = self.check_new_room();

                true
            }
            None => false,
        }
    }

    crate fn add_newuser(&mut self, info: &LoginInfo) -> UserID {
        let id = self.create_user(info);

        self.waitings.push_user(id);

        let _ = self.check_new_room();

        id
    }

    crate fn remove_user(&mut self, id: &UserID) -> bool {
        match self.users.remove(id) {
            Some(_) => {
                let remove_ids = self
                    .rooms
                    .iter()
                    .filter(|x| x.1.contains(&id))
                    .map(|x| x.0.clone())
                    .collect::<Vec<_>>()
                    .into_iter();

                for r_id in remove_ids {
                    self.rooms.remove(&r_id);
                }

                self.waitings.remove_user(id);

                true
            }
            None => false,
        }
    }

    crate fn add_ships(&mut self, id: &UserID, ships: &Vec<Ship>) -> Result<(), String> {
        let room_id = self.room_id_of_user(id)?;
        let room = self.room_mut(&room_id).ok_or("Room not found")?;
        let user_kind = room.userkind_by_id(id).expect("User is not found in Room");
        room.add_ships(&user_kind, ships);
        Ok(())
    }

    crate fn check_room_is_finished(
        &self,
        id: &RoomID,
        receiver: &UserKind,
    ) -> Result<bool, String> {
        let room = self.room(id).ok_or("Room not found")?;

        let receiver_user = room.user(receiver);

        Ok(receiver_user.ships.len() == 0)
    }

    crate fn attack(&mut self, attack: Attack) -> Result<(), String> {
        let attacker_id = attack.attacker_id;
        let room_id = self.room_id_of_user(&attacker_id)?;
        let room = self.room_mut(&room_id).ok_or("Room not found")?;

        let kind_attacker = room
            .userkind_by_id(&attacker_id)
            .ok_or("User is not found in Room")?;

        let kind_target = kind_attacker.rev();

        let target_id = room.user(&kind_target).id;

        let (attacker_result, receiver_result) = room.attack(&kind_target, attack);

        self.user_push_json(&attacker_id, &attacker_result)?;
        self.user_push_json(&target_id, &receiver_result)?;

        let is_finished = self.check_room_is_finished(&room_id, &kind_target)?;

        if is_finished {
            self.user_push_json(&attacker_id, &GameFinish::new(true))?;
            self.user_push_json(&target_id, &GameFinish::new(false))?;

            let _ = self.remove_room(&room_id);
        }
        // let receiver_ships_count = room.user(&kind_target).ships.len();

        Ok(())
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
            let id = model.add_newuser(&super::LoginInfo {
                username: format!("hoge_{}", i),
            });
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