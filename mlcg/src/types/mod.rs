pub mod building;
pub mod number;
pub mod unit;

pub trait Type {
    fn from_name(name: crate::String) -> Self;
}
