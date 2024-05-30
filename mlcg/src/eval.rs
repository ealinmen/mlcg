pub use mlcg_derive::Eval;

pub trait Eval<T> {
    fn eval(self) -> T;
}
