use rocket::{
    State,
};

use rocket_contrib::json::Json;

use crate::{
    state::{
        mystate::{MyState},
    },
    json::{
        JsonResult,
        game::{
            Ships,
            Attack,
        },
    }
};

#[post("/add_ships", data = "<data>")]
pub fn add_ships(data: Json<Ships>, state: State<MyState>) -> Json<JsonResult<(), String>> {
    Json(state.add_ships(&data.0).into())
}

#[post("/attack", data = "<data>")]
pub fn attack(data: Json<Attack>, state: State<MyState>) -> Json<JsonResult<(), String>> {
    Json(state.attack(data.0).into())
}