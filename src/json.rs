// use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(tag = "tag", content = "content")]
pub enum JsonResult<T, E> {
    Ok(T),
    Err(E),
}



pub mod login;
pub mod game;