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
}

impl Model {
    crate fn new() -> Model {
        Model {
            next_user_id : 0,
            users : HashMap::new(),
            waitings : WaitingUsers::new(),
        }
    }

    crate fn add_newuser(&mut self) -> u32 {
        let id = self.next_user_id;
        self.next_user_id += 1u32;
        self.waitings.get_shorter().push(id);

        let user = User { id };
        self.users.insert(id, user);

        id
    }
}