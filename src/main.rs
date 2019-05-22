#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

extern crate rocket_contrib;

use rocket::{
    State,
};

use rocket_contrib::json::Json;

// extern crate kaisendon_saba;
use kaisendon_saba::{
    state::{
        mystate::{MyState},
        // user::{User},
    },
    json::{
        login::{LoginInfo, LoginResult},
    }
};

mod debug {
    #[get("/model")]
    pub fn model(state: super::State<super::MyState>) -> String {
        println!("Model:\n{}", state.model_string());
        format!("Debug Printed at server.")
    }
}

#[get("/")]
fn index(state: State<MyState>) -> String {
    format!("State: {:?}", state)
}


#[post("/login", data = "<info>")]
fn login(info: Json<LoginInfo>, state: State<MyState>) -> Json<LoginResult> {
    match state.add_newuser(&info.0) {
        Ok(user) =>
            Json(LoginResult::success(user)),
        Err(e) => {
            dbg!(e.clone());
            Json(LoginResult::error(e.to_string()))
        },
    }
}


fn main() {
    let mystate = MyState::new();

    rocket::ignite()
        .mount("/", routes![index, login])
        .mount("/debug", routes![debug::model])
        .manage(mystate)
        .launch();
}