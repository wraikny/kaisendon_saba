use super::super::{
    error::{ Error },
};

use super::{
    user::{UserID, User, WaitingUsers},
    room::{Room},
};

use super::super::json::{
    login::LoginInfo,
};

use std::collections::{
    HashMap,
};

#[derive(Debug)]
crate struct Model {
    crate next_user_id : UserID,
    crate users : HashMap<UserID, User>,
    crate waitings : WaitingUsers,

    // crate next_room_id : u32,
    // crate rooms : HashMap<u32, Room>,
}

impl Model {
    crate fn new() -> Model {
        Model {
            next_user_id : 0,
            users : HashMap::new(),
            waitings : WaitingUsers::new(),

            // next_room_id : 0,
            // rooms : HashMap::new(),
        }
    }

    crate fn add_newuser(&mut self, info : &LoginInfo) -> User {
        let id = self.next_user_id;
        self.next_user_id += 1;
        self.waitings.get_shorter().push(id);

        let user = User::new(id, &info.username);
        self.users.insert(id, user.clone());

        user
    }
}