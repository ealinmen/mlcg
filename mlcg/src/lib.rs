#![allow(incomplete_features)]
#![feature(specialization)]

pub mod command {

    mlcg_derive::commands!("src/commands.json");
}

pub mod abilities;
pub mod env;
pub mod eval;
pub mod processor;
#[macro_use]
pub mod r#ref;
pub mod types;

mod string;
use string::String;
