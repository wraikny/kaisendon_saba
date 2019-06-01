use rocket::{
    State,
};

use rocket_contrib::json::Json;

use crate::{
    state::{
        user::{
            UserID,
        },
        mystate::{MyState},
        // user::{User},
        // setting::{Setting},
    },
    json::{
        User,
        JsonResult,
        LoginInfo,
        UserInfo,
        UserWaitingState,
    },
    // game::{
    //     RoomID,
    // },
};

#[get("/")]
pub fn index() -> String {
    format!("Welcome to kaisendon")
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

#[post("/user", data = "<info>")]
pub fn get_user(info: Json<UserInfo>, state: State<MyState>) -> Json<JsonResult<User, String>> {
    Json(state.get_user_json(&info.id).into())
}

#[post("/queue", data = "<info>")]
pub fn jsons_queue(info: Json<UserInfo>, state: State<MyState>) -> Json<JsonResult<Vec<String>, String>> {
    Json(state.user_pop_jsons(&info.id).into())
}