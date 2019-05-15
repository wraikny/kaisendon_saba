#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;

extern crate kaisendon_saba;
use kaisendon_saba::state::{
    model::{Model},
    mystate::{MyState},
};

#[get("/")]
fn index(state: State<MyState>) -> String {
    let mut model = state.model().unwrap();
    model.hoge += 1;
    format!("The config value is: {:?}", model)
}

fn main() {
    let model = Model {
        user_val: "user input".to_string(),
        hoge : 0,
    };

    rocket::ignite()
        .mount("/", routes![index])
        .manage(MyState::new(model))
        .launch();
}