use super::super::{
    state::{
        user::{UserID},
    }
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfo {
    pub username: String
}

pub mod result {
    pub const SUCCESS: i32 = 0;
    pub const ERROR: i32 = 1;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    pub id: UserID,
    pub error: i32,
    pub error_msg: String,
}

impl LoginResult {
    pub fn success(id: UserID) -> LoginResult {
        LoginResult {
            id: id,
            error: result::SUCCESS,
            error_msg: "".to_owned(),
        }
    }

    pub fn error(msg: String) -> LoginResult {
        LoginResult {
            id: 0,
            error: result::ERROR,
            error_msg: msg,
        }
    }
}