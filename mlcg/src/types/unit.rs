use crate::abilities::*;
use crate::env::*;
use crate::String;
use std::marker::PhantomData;

use super::Type;

pub struct Unit<U: Units> {
    pub name: String,
    _unit: PhantomData<U>,
}

impl<U: Units> Unit<U> {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            _unit: PhantomData,
        }
    }

    pub fn class_name(&self) -> &'static str {
        U::class_name()
    }
}

impl<U: Units> Type for Unit<U> {
    fn from_name(name: crate::String) -> Self {
        Self::new(name)
    }
}

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
