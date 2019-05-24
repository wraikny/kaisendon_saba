use super::{
    Cell,
    AttackPoint
};

#[derive(Debug, Clone)]
crate struct Ship {
    crate hp : i32,
    crate cells : Vec<Cell>,
}