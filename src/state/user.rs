use crate::{
    game::{
        RoomID
    },
};

use serde::{Serialize};

use std::collections::{VecDeque};

pub type UserID = u32;

#[derive(Debug, Clone)]
pub struct User {
    pub id : UserID,
    pub name : String,
    crate room_id : Option<u32>,

    jsons : VecDeque<String>,
}

impl User {
    crate fn new(id : UserID, name : &str) -> Self {
        User {
            id : id,
            name : name.to_owned(),
            room_id : None,
            jsons : VecDeque::new(),
        }
    }

    crate fn set_room(&mut self, id: RoomID) {
        self.room_id = Some(id);
    }

    crate fn push_json<T : Serialize>(&mut self, obj : &T) -> Result<(), String> {
        let json = serde_json::to_string(obj).map_err(|e| format!("{}", e))?;
        self.jsons.push_back(json);
        Ok(())
    }

    crate fn pop_jsons(&mut self) -> Vec<String> {
        let result = self.jsons.clone().into_iter().collect();
        
        self.jsons.clear();

        result
    }
}

#[derive(Debug)]
crate struct WaitingUsers {
    winners : Vec<UserID>,
    losers : Vec<UserID>,
}

impl WaitingUsers {
    crate fn new() -> Self {
        WaitingUsers {
            winners : Vec::new(),
            losers : Vec::new(),
        }
    }

    crate fn larger(&self) -> &Vec<UserID> {
        if self.losers.len() >= self.winners.len() {
            &self.losers
        } else {
            &self.winners
        }
    }

    crate fn larger_mut(&mut self) -> &mut Vec<UserID> {
        if self.losers.len() >= self.winners.len() {
            &mut self.losers
        } else {
            &mut self.winners
        }
    }

    crate fn remove_user(&mut self, user: &UserID) {
        self.winners = self.winners.clone().into_iter().filter(|x| *x != *user).collect();
        self.losers = self.losers.clone().into_iter().filter(|x| *x != *user).collect();
    }
}