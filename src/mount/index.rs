use rocket::{
    State,
};

use rocket_contrib::json::Json;

// extern crate kaisendon_saba;
use super::super::{
    state::{
        user::{
            UserID,
        },
        mystate::{MyState},
        // user::{User},
        // setting::{Setting},
    },
    json::{
        JsonResult,
        LoginInfo,
        UserInfo,
        UserWaitingState,
    },
    game::{
        RoomID,
    },
};

#[get("/")]
pub fn index(state: State<MyState>) -> String {
    format!("State: {:?}", state)
}


#[post("/login", data = "<info>")]
pub fn login(info: Json<LoginInfo>, state: State<MyState>) -> Json<JsonResult<UserID, String>> {
    match state.add_newuser(&info.0) {
        Ok(id) =>
            Json(JsonResult::Ok(id)),
        Err(e) => {
            dbg!(e.clone());
            Json(JsonResult::Err(e.to_string()))
        },
    }
}

#[post("/logout", data = "<info>")]
pub fn logout(info: Json<UserInfo>, state: State<MyState>) -> Json<JsonResult<bool, String>> {
    match state.remove_user(&info.id) {
        Ok(result) =>
            Json(JsonResult::Ok(result)),
        Err(e) => {
            dbg!(e.clone());
            Json(JsonResult::Err(e.to_string()))
        },
    }
}



#[post("/waiting", data = "<info>")]
pub fn waiting(info: Json<UserInfo>, state: State<MyState>) -> Json<JsonResult<UserWaitingState, String>> {
    Json(state.check_waiting(&info.id).into())
}