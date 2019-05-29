#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

extern crate serde_json;

use rocket::{
    State,
};

use rocket_contrib::json::Json;

// extern crate kaisendon_saba;
use kaisendon_saba::{
    state::{
        user::{
            UserID,
        },
        mystate::{MyState},
        // user::{User},
        setting::{Setting},
    },
    json::{
        JsonResult,
        login::{
            LoginInfo,
            LogoutInfo,
        },
    }
};

mod debug {
    #[get("/model")]
    pub fn model(state: super::State<super::MyState>) -> String {
        state.print_model();
        format!("Debug Printed at server.")
    }
}

#[get("/")]
fn index(state: State<MyState>) -> String {
    format!("State: {:?}", state)
}


#[post("/login", data = "<info>")]
fn login(info: Json<LoginInfo>, state: State<MyState>) -> Json<JsonResult<UserID, String>> {
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
fn logout(info : Json<LogoutInfo>, state: State<MyState>) -> Json<JsonResult<bool, String>> {
    match state.remove_user(&info.id) {
        Ok(result) =>
            Json(JsonResult::Ok(result)),
        Err(e) => {
            dbg!(e.clone());
            Json(JsonResult::Err(e.to_string()))
        },
    }
}



fn main() {
    let setting = Setting::new( (12, 12) );
    let mystate = MyState::new(setting);

    rocket::ignite()
        .mount("/", routes![index, login])
        .mount("/debug", routes![debug::model])
        .manage(mystate)
        .launch();
}

#[cfg(test)]
mod test {
    #[test]
    fn result_json() {
        let r : Result<i32, String> = Ok(0);
        let e : Result<i32, String> = Err("error".to_owned());

        let json_r = super::Json(r);
        let json_e = super::Json(e);
        println!("{:?}", json_r.into_string());
        println!("{:?}", json_e);
    }
}