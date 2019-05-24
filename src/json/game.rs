use super::super::game::{
    Cell,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum AttackKind {
    Destroy,
    Hit,
    Fail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttackResult {
    pub cells : Vec<(Cell, AttackKind)>
}