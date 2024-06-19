use crate::eval::Eval;
use crate::String;

pub mod building;
pub mod config;
pub mod number;
pub mod unit;

pub trait Type: Eval<String> {
    fn from_name(name: crate::String) -> Self;
}
