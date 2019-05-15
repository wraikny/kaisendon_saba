#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;

// extern crate kaisendon_saba;
use kaisendon_saba::state::{
    mystate::{MyState},
};

#[get("/")]
fn index(state: State<MyState>) -> String {
    format!("The config value is: {:?}", state)
}

fn main() {
    let mystate = MyState::new();

    rocket::ignite()
        .mount("/", routes![index])
        .manage(mystate)
        .launch();
}