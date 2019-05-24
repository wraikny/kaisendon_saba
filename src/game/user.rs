use super::{
    Cell, ShipID,
    ship::{Ship},
};

use super::super::{
    state::{
        user::{UserID},
    },
    json::{
        game::{Attack, AttackKind, AttackkerResult, ReceiverResult},
    }
};

use std::collections::{
    HashMap,
};

#[derive(Debug)]
crate struct User {
    crate id : UserID,
    crate cells : HashMap<Cell, ShipID>,

    crate ships : HashMap<ShipID, Ship>
}

impl User {
    crate fn new(id : &UserID) -> User {
        User {
            id : id.clone(),
            cells : HashMap::new(),

            ships : HashMap::new(),
        }
    }

    fn add_ship(&mut self, ship : &Ship) {
        let id = ship.id;
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

    crate fn receive_attack(&mut self, attack : Attack) -> (AttackkerResult, ReceiverResult) {
        let mut results = Vec::new();
        let mut destroyed_ships = Vec::new();

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
                        destroyed_ships.push(ship_id);
                    } else {
                        results.push((*cell, AttackKind::Hit));
                    }
                },
                None => {
                    results.push((*cell, AttackKind::Fail));
                },
            }
        }

        ( AttackkerResult { cells : results }
        , ReceiverResult {attack, destroyed_ships})
    }
}