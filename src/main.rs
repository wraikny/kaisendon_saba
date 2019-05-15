#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use std::sync::{Arc, Mutex};

extern crate kaisendon_saba;
use kaisendon_saba::model::{Model};

struct MyState {
    model : Arc<Mutex<Model>>
}

impl MyState {
    fn new(model : Model) -> MyState {
        MyState {
            model : Arc::new(Mutex::new(model))
        }
    }
}


#[get("/")]
fn index(state: State<MyState>) -> String {
    let mut model = state.model.lock().unwrap();
    model.hoge[0] += 1;
    format!("The config value is: {:?}", model)
}

fn main() {
    let model = Model {
        user_val: "user input".to_string(),
        hoge : vec![1, 2, 3],
    };

    rocket::ignite()
        .mount("/", routes![index])
        .manage(MyState::new(model))
        .launch();
}