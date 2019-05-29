// use serde::{Deserialize, Serialize};
use super::{
    state::{
        user::{UserID},
    },
    game::{RoomID},
};

#[derive(Debug, Serialize)]
#[serde(tag = "tag", content = "content")]
pub enum JsonResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> From<Result<T, E>> for JsonResult<T, String> 
    where E : std::string::ToString,
{
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(t) => JsonResult::Ok(t),
            Err(e) => JsonResult::Err(e.to_string()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub id: UserID,
}

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: UserID,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Room {
    pub id: RoomID,
    pub user1: User,
    pub user2: User,
}

#[derive(Debug, Serialize)]
#[serde(tag = "tag", content = "content")]
pub enum UserWaitingState {
    InRoom(Room),
    Waiting,
}

pub mod game;