use crate::{
    game::{
        Cell, ShipID, ship::Ship, AttackPoint
    },
    state::{
        user::{
            UserID,
        }
    },
};

#[derive(Debug, Deserialize)]
pub struct Ships {
    pub id: UserID,
    pub ships: Vec<Ship>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Attack {
    pub attacker_id : UserID,
    pub cells : Vec<(Cell, AttackPoint)>,
}

#[derive(Debug, Serialize)]
pub enum AttackKind {
    Destroy,
    Hit,
    Fail,
}

#[derive(Debug, Serialize)]
pub struct AttackkerResult {
    pub cells : Vec<(Cell, AttackKind)>
}

#[derive(Debug, Serialize)]
pub struct ReceiverResult {
    pub attack : Attack,
    pub destroyed_ships : Vec<ShipID>,
}

#[derive(Debug, Serialize)]
pub struct GameFinish {
    pub is_win: bool,
}

impl GameFinish {
    crate fn new(is_win: bool) -> Self {
        GameFinish { is_win }
    }
}