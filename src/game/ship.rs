use super::{
    Cell, ShipID,
};

#[derive(Debug, Clone)]
crate struct Ship {
    crate id : ShipID,
    crate hp : i32,
    crate cells : Vec<Cell>,
}