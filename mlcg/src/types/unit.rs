use crate::abilities::*;
use crate::command::ucontrol;
use crate::env::*;
use crate::eval::Eval;
use crate::r#ref::Ref;
use crate::String;
use std::marker::PhantomData;

use super::building::{Building, Buildings};
use super::number::Number;
use super::Type;

#[derive(Eval, Debug, Clone)]
pub struct Unit<U: Units = Binding> {
    pub(crate) name: String,
    pub(crate) _unit: PhantomData<U>,
}

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

impl<'a, U: Units> Ref<'a, Unit<U>> {
    pub fn bind(&self) -> Ref<'a, Unit> {
        let s = self.cast::<Unit>();
        self.core.unit().set_to(s);
        s
    }
}

impl<'a> Ref<'a, Unit> {
    fn ucontrol(&self, command: impl Into<ucontrol::Ucontrol>) {
        self.core.borrow_mut().push_command(command.into());
    }

    pub fn idle(&self) {
        self.ucontrol(ucontrol::Idle {});
    }

    pub fn move_(&self, x: impl Eval<Number>, y: impl Eval<Number>) {
        assert_same_core!(self, x, y);
        let command = ucontrol::Move {
            x: x.eval().eval(),
            y: y.eval().eval(),
        };
        self.ucontrol(command);
    }

    pub fn approach(&self, x: impl Eval<Number>, y: impl Eval<Number>, radius: impl Eval<Number>) {
        assert_same_core!(self, x, y, radius);
        let command = ucontrol::Approach {
            x: x.eval().eval(),
            y: y.eval().eval(),
            radius: radius.eval().eval(),
        };
        self.ucontrol(command);
    }

    pub fn path_find(&self, x: impl Eval<Number>, y: impl Eval<Number>) {
        assert_same_core!(self, x, y);
        let command = ucontrol::PathFind {
            x: x.eval().eval(),
            y: y.eval().eval(),
        };
        self.ucontrol(command);
    }

    pub fn auto_path_find(&self) {
        self.ucontrol(ucontrol::AutoPathFind {});
    }

    pub fn boost(&self, enable: impl Eval<Number>) {
        assert_same_core!(self, enable);
        let command = ucontrol::Boost {
            enable: enable.eval().eval(),
        };
        self.ucontrol(command);
    }

    #[doc(alias = "shoot")]
    pub fn target(&self, x: impl Eval<Number>, y: impl Eval<Number>, shoot: impl Eval<Number>) {
        assert_same_core!(self, x, y, shoot);
        let command = ucontrol::Target {
            x: x.eval().eval(),
            y: y.eval().eval(),
            shoot: shoot.eval().eval(),
        };
        self.ucontrol(command);
    }

    #[doc(alias = "shootp")]
    pub fn targetp<At: Target>(&self, at: impl Eval<At>, shoot: impl Eval<Number>) {
        assert_same_core!(self, at, shoot);
        let command = ucontrol::Targetp {
            unit: at.eval().eval(),
            shoot: shoot.eval().eval(),
        };
        self.ucontrol(command);
    }

    pub fn item_drop<B: Buildings>(&self, to: impl Eval<Building<B>>, amount: impl Eval<Number>) {
        assert_same_core!(self, to, amount);
        let command = ucontrol::ItemDrop {
            to: to.eval().eval(),
            amount: amount.eval().eval(),
        };
        self.ucontrol(command);
    }

    pub fn item_take<B: Buildings>(
        &self,
        from: impl Eval<Building<B>>,
        item: impl Eval<String>,
        amount: impl Eval<Number>,
    ) {
        assert_same_core!(self, from, item, amount);
        let command = ucontrol::ItemTake {
            from: from.eval().eval(),
            item: item.eval(),
            amount: amount.eval().eval(),
        };
        self.ucontrol(command);
    }

    pub fn pay_drop(&self) {
        let command = ucontrol::PayDrop {};
        self.ucontrol(command);
    }

    pub fn pay_take(&self, take_units: impl Eval<Number>) {
        assert_same_core!(self, take_units);
        let command = ucontrol::PayTake {
            take_units: take_units.eval().eval(),
        };
        self.ucontrol(command);
    }

    pub fn pay_enter(&self) {
        let command = ucontrol::PayEnter {};
        self.ucontrol(command);
    }

    pub fn mine(&self, x: impl Eval<Number>, y: impl Eval<Number>) {
        assert_same_core!(self, x, y);
        let command = ucontrol::Mine {
            x: x.eval().eval(),
            y: y.eval().eval(),
        };
        self.ucontrol(command);
    }

    pub fn flag(&self, value: impl Eval<Number>) {
        assert_same_core!(self, value);
        let command = ucontrol::Flag {
            value: value.eval().eval(),
        };
        self.ucontrol(command);
    }

    // build

    // getblock

    pub fn within(
        &self,
        x: impl Eval<Number>,
        y: impl Eval<Number>,
        radius: impl Eval<Number>,
    ) -> Ref<'_, Number> {
        assert_same_core!(self, x, y, radius);
        let result = self.core.new_unnamed();
        let command = ucontrol::Within {
            x: x.eval().eval(),
            y: y.eval().eval(),
            radius: radius.eval().eval(),
            result: result.eval(),
        };
        self.ucontrol(command);
        result
    }

    pub fn unbind(&self) {
        let command = ucontrol::Unbind {};
        self.ucontrol(command);
    }
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

pub type AnyUnit = Binding;

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
