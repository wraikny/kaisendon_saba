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

    crate fn get_longer(&mut self) -> &mut Vec<UserID> {
        if self.losers.len() >= self.winners.len() {
            &mut self.losers
        } else {
            &mut self.winners
        }
    }

    crate fn get_shorter(&mut self) -> &mut Vec<UserID> {
        if self.losers.len() < self.winners.len() {
            &mut self.losers
        } else {
            &mut self.winners
        }
    }
}