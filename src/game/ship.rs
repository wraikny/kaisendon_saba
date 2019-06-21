use super::{Cell, ShipID};

#[derive(Debug, Clone, Deserialize)]
pub struct Ship {
    crate id: ShipID,
    crate hp: i32,
    crate cells: Vec<Cell>,
}