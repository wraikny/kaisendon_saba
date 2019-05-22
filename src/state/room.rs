// use super::super::{
//     error::{ Error },
// };

use super::{
    user::{UserID},
};


pub type RoomID = u32;


#[derive(Debug)]
crate struct Room {
    crate id: RoomID,
    crate user1 : UserID,
    crate user2 : UserID,
}

impl Room {
    crate fn new(id: RoomID, user1: UserID, user2: UserID) -> Room {
        Room{ id, user1, user2 }
    }

    crate fn contains(&self, user: &UserID) -> bool {
        self.user1 == *user || self.user2 == *user
    }
}