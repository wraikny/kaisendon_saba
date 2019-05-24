use super::super::game::{
    Cell, AttackPoint
};

#[derive(Debug, Deserialize)]
crate struct Attack {
    crate cells : Vec<(Cell, AttackPoint)>,
}

#[derive(Debug, Serialize)]
pub enum AttackKind {
    Destroy,
    Hit,
    Fail,
}

#[derive(Debug, Serialize)]
pub struct AttackResult {
    pub cells : Vec<(Cell, AttackKind)>
}