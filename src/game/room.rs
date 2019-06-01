use crate::{
    game::{ship::Ship, user::User, RoomID},
    json::game::{Attack, AttackkerResult, ReceiverResult},
    state::user::UserID,
};

crate enum UserKind {
    User1,
    User2,
}

impl UserKind {
    crate fn rev(&self) -> Self {
        match *self {
            UserKind::User1 => UserKind::User2,
            UserKind::User2 => UserKind::User1,
        }
    }
}


#[derive(Debug, Clone)]
crate struct Room {
    crate id: RoomID,
    crate user1: User,
    crate user2: User,
}

impl Room {
    crate fn new(id: RoomID, user1: &UserID, user2: &UserID) -> Room {
        Room {
            id,
            user1: User::new(user1),
            user2: User::new(user2),
        }
    }

    crate fn contains(&self, user: &UserID) -> bool {
        self.user1.id == *user || self.user2.id == *user
    }

    crate fn user(&self, kind: &UserKind) -> &User {
        match *kind {
            UserKind::User1 => &self.user1,
            UserKind::User2 => &self.user2,
        }
    }

    fn user_mut(&mut self, kind: &UserKind) -> &mut User {
        match *kind {
            UserKind::User1 => &mut self.user1,
            UserKind::User2 => &mut self.user2,
        }
    }

    crate fn userkind_by_id(&mut self, id: &UserID) -> Option<UserKind> {
        if self.user1.id == *id {
            Some(UserKind::User1)
        } else if self.user2.id == *id {
            Some(UserKind::User2)
        } else {
            None
        }
    }

    crate fn add_ships(&mut self, kind: &UserKind, ships: &Vec<Ship>) {
        let user = self.user_mut(kind);
        user.add_ships(ships);
    }

    crate fn attack(
        &mut self,
        target: &UserKind,
        attack: Attack,
    ) -> (AttackkerResult, ReceiverResult) {
        let user = self.user_mut(target);
        user.receive_attack(attack)
    }
}