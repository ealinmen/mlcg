use crate::abilities::*;
use crate::env::*;
use crate::eval::Eval;
use crate::String;
use std::marker::PhantomData;

use super::Type;
mod inner {
    use super::*;

    #[derive(Eval, Debug, Clone)]
    pub struct Unit<U: Units> {
        pub(crate) name: String,
        pub(crate) _unit: PhantomData<U>,
    }
}

pub type Unit<U = Binding> = inner::Unit<U>;

impl<U: Units> Unit<U> {
    pub fn class_name(&self) -> &'static str {
        U::class_name()
    }
}

impl<U: Units> Type for Unit<U> {
    fn from_name(name: crate::String) -> Self {
        Self {
            name,
            _unit: PhantomData,
        }
    }
}

impl<U: Units> Eval<String> for Unit<U> {
    fn eval(self) -> String {
        self.name
    }
}

impl<U: Units> Target for Unit<U> {}

pub trait Units {
    fn class_name() -> &'static str;
}

pub trait Land: Units + Sepro {}
pub trait Air: Units + Sepro {}
pub trait Naval: Units + Sepro {}

pub trait Attack: Units + Sepro {}

pub trait Support: Units + Sepro {}

pub trait Legs: Units + Sepro + Land {}

pub trait Tank: Units + Erekir {}

pub trait Mech: Units + Erekir {}

pub trait Flying: Units + Erekir {}

pub trait Neoplasm: Units + Erekir {}

pub trait Core: Units {}

pub trait Internal: Units {}

macro_rules! units {
    ($(
        $unit:ident => $class:literal : $($trait: ident),+ && $($ability : ident),*
    ;)*) => {$(
        pub struct $unit;

        impl Units for $unit {
            fn class_name() -> &'static str {
                concat!('@', $class)
            }
        }

        $(
        impl $trait for $unit {}
        )*

        $(
        impl $ability for $unit {}
        impl $ability for Unit<$unit> {}
        )*

    )*}
}

units! {
    Binding  => "unit"     : Sepro, Erekir, Land, Air, Naval, Support, Legs, Tank, Flying, Neoplasm, Core, Internal && Shoot, Boost;

    Dagger   => "dagger"   : Sepro, Land, Attack && Shoot;
    Mace     => "mace"     : Sepro, Land, Attack && Shoot;
    Fortress => "fortress" : Sepro, Land, Attack && Shoot;
    Scepter  => "scepter"  : Sepro, Land, Attack && Shoot;
    Reign    => "reign"    : Sepro, Land, Attack && Shoot;

    Nova     => "nova"     : Sepro, Land, Support && Shoot, Boost;
    Pulsar   => "pulsar"   : Sepro, Land, Support && Shoot, Boost;
    Quasar   => "quasar"   : Sepro, Land, Support && Shoot, Boost;
    Vela     => "vela"     : Sepro, Land, Support && Shoot, Boost;
    Corvus   => "corvus"   : Sepro, Land, Support && Shoot, Boost;
}
