use super::{
    room::{RoomID},
};

pub type UserID = u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id : UserID,
    pub name : String,
    crate room_id : Option<u32>,
}

impl User {
    crate fn new(id : UserID, name : &str) -> Self {
        User {
            id : id,
            name : name.to_owned(),
            room_id : None,
        }
    }

    crate fn set_room(&mut self, id: RoomID) {
        self.room_id = Some(id);
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