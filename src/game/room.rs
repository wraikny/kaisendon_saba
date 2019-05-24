// use super::super::{
//     error::{ Error },
// };

use super::{
    RoomID,
    ship::{Ship},
    user::{User},
};

use super::super::{
    state::{
        user::{UserID},
    },
    json::{
        game::{Attack, AttackResult},
    }
};

crate enum UserKind {
    User1,
    User2,
}


#[derive(Debug)]
crate struct Room {
    crate id: RoomID,
    crate user1 : User,
    crate user2 : User,
}

impl Room {
    crate fn new(id: RoomID, user1: &UserID, user2: &UserID) -> Room {
        Room {
            id,
            user1 : User::new(user1),
            user2 : User::new(user2),
        }
    }

    crate fn contains(&self, user: &UserID) -> bool {
        self.user1.id == *user || self.user2.id == *user
    }

    fn get_user(&mut self, kind : &UserKind) -> &mut User {
        match *kind {
            UserKind::User1 => &mut self.user1,
            UserKind::User2 => &mut self.user2,
        }
    }

    crate fn add_ships(&mut self, kind : &UserKind, ships : &Vec<Ship>) {
        let user = self.get_user(kind);
        user.add_ships(ships);
    }

    crate fn attack(&mut self, target : &UserKind, attack : &Attack) -> AttackResult {
        let user = self.get_user(target);
        user.receive_attack(attack)
    }
}