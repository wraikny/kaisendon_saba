use super::super::{
    state::{
        user::{UserID},
    }
};

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    pub username: String
}

#[derive(Debug, Deserialize)]
pub struct LogoutInfo {
    pub id: UserID
}