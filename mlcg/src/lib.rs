#![allow(incomplete_features)]
#![feature(specialization)]
pub mod abilities;
pub mod command {

    mlcg_derive::commands!("src/commands.json");
}
pub mod env;
pub mod eval;
pub mod processor;
pub mod r#ref;
pub mod types;

mod string;
use string::String;
