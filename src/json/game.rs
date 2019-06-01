use crate::{
    game::{
        Cell, ShipID, AttackPoint
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Attack {
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