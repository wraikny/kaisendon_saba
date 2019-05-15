#[derive(Debug, Serialize, Deserialize)]
crate struct User {
    crate id : u32
}

#[derive(Debug)]
crate struct WaitingUsers {
    winners : Vec<u32>,
    losers : Vec<u32>,
}

impl WaitingUsers {
    crate fn new() -> Self {
        WaitingUsers {
            winners : Vec::new(),
            losers : Vec::new(),
        }
    }

    crate fn get_longer(&mut self) -> &mut Vec<u32> {
        if self.losers.len() >= self.winners.len() {
            &mut self.losers
        } else {
            &mut self.winners
        }
    }

    crate fn get_shorter(&mut self) -> &mut Vec<u32> {
        if self.losers.len() < self.winners.len() {
            &mut self.losers
        } else {
            &mut self.winners
        }
    }
}