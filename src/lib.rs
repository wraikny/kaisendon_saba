#![feature(crate_visibility_modifier)]
#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

extern crate failure;

extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod state;
crate mod game;
pub mod json;
pub mod error;
pub mod mount;