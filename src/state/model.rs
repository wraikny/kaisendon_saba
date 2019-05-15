use super::{
    user::{User, WaitingUsers},
    room::{Room},
    error::{ Error },
};

use std::collections::{
    HashMap,
};

#[derive(Debug)]
crate struct Model {
    crate next_user_id : u32,
    crate users : HashMap<u32, User>,
    crate waitings : WaitingUsers,

    crate next_room_id : u32,
    crate rooms : HashMap<u32, Room>,
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

    crate fn add_newuser(&mut self) -> u32 {
        let id = self.next_user_id;
        self.next_user_id += 1u32;
        self.waitings.get_shorter().push(id);

        let user = User::new(id);
        self.users.insert(id, user);

        id
    }
}