use super::{
    Cell, ShipID,
    ship::{Ship, Attack},
};

use super::super::{
    state::{
        user::{UserID},
    },
    json::{
        game::{AttackKind, AttackResult},
    }
};

use std::collections::{
    HashMap,
};

#[derive(Debug)]
crate struct User {
    crate id : UserID,
    crate cells : HashMap<Cell, ShipID>,

    crate next_ship_id : ShipID,
    crate ships : HashMap<ShipID, Ship>
}

impl User {
    crate fn new(id : &UserID) -> User {
        User {
            id : id.clone(),
            cells : HashMap::new(),

            next_ship_id : 0,
            ships : HashMap::new(),
        }
    }

    fn add_ship(&mut self, ship : &Ship) {
        let id = self.next_ship_id;
        self.next_ship_id += 1;

        self.ships.insert(id, ship.clone());
        for cell in ship.cells.iter() {
            self.cells.insert(cell.clone(), id);
        }
    }

    crate fn add_ships(&mut self, ships : &Vec<Ship>) {
        for ship in ships {
            self.add_ship(ship)
        }
    }

    crate fn receive_attack(&mut self, attack : &Attack) -> AttackResult {
        let mut results = Vec::new();

        for (cell, p) in attack.cells.iter() {
            match self.cells.get(cell).cloned() {
                Some(ship_id) => {
                    let ship = self.ships.get_mut(&ship_id).unwrap();
                    ship.hp -= p;
                    if ship.hp <= 0 {
                        for cell in ship.cells.iter() {
                            self.cells.remove(cell);
                        }
                        self.ships.remove(&ship_id);

                        results.push((*cell, AttackKind::Destroy));
                    } else {
                        results.push((*cell, AttackKind::Hit));
                    }
                },
                None => {
                    results.push((*cell, AttackKind::Fail));
                },
            }
        }

        AttackResult { cells : results }
    }
}