pub use mlcg_derive::*;

use crate::processor::Processor;

pub trait WithCore {
    fn core(&self) -> Option<&Processor>;
}

impl<T> WithCore for T {
    default fn core(&self) -> Option<&Processor> {
        None
    }
}

pub trait Eval<T>: WithCore {
    fn eval(self) -> T;
}
