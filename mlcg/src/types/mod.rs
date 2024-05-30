pub mod number;

pub trait Type {
    fn from_name(name: crate::String) -> Self;
}
