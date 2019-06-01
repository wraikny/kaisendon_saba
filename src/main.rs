#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

extern crate serde_json;

use rocket::{
    State,
};

// extern crate kaisendon_saba;
use kaisendon_saba::{
    state::{
        mystate::{MyState},
        // user::{User},
        setting::{Setting},
    },
    mount::{
        index::*,
        game,
    }
};

mod debug {
    #[get("/model")]
    pub fn model(state: super::State<super::MyState>) -> String {
        state.print_model();
        format!("Debug Printed at server.")
    }
}


fn main() {
    let setting = Setting::new( (12, 12) );
    let mystate = MyState::new(setting);

    rocket::ignite()
        .mount("/", routes![index, login, logout, waiting, user, pop_queue])
        .mount("/game", routes![game::add_ships, game::attack])
        .mount("/debug", routes![debug::model])
        .manage(mystate)
        .launch();
}