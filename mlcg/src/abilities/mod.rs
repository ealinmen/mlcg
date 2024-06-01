use crate::{eval::Eval, String};

pub trait Shoot {}

pub trait Target : Eval<String>{}

pub trait Boost {}
