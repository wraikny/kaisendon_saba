use super::super::{
    game::{
        Cell
    },
};

#[derive(Debug)]
pub struct Setting {
    crate board_size : Cell
}

impl Setting {
    pub fn new(board_size : Cell) -> Setting {
        Setting {
            board_size
        }
    }
}