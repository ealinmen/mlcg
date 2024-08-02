use super::{number::Number, Type};
use crate::{
    abilities::{Shoot, Target},
    command,
    eval::Eval,
    r#ref::Ref,
    String,
};
use std::marker::PhantomData;

#[derive(Eval, Debug, Clone)]
pub struct Building<B: Buildings = AnyBuilding> {
    name: String,
    _building: PhantomData<B>,
}

impl<B: Buildings> Type for Building<B> {
    fn from_name(name: crate::String) -> Self {
        Self {
            name,
            _building: PhantomData,
        }
    }
}

impl<B: Buildings> Eval<String> for Building<B> {
    fn eval(self) -> String {
        self.name
    }
}

pub trait Buildings {
    fn class_name() -> &'static str;
}

pub struct AnyBuilding;

impl Buildings for AnyBuilding {
    fn class_name() -> &'static str {
        panic!("unknow building")
    }
}

impl<'a, T> Ref<'a, T>
where
    T: Type + Shoot + Buildings,
{
    #[doc(alias = "target")]
    pub fn shoot(&self, x: impl Eval<Number>, y: impl Eval<Number>, shoot: impl Eval<Number>) {
        assert_same_core!(self, x, y, shoot);
        let command = command::ucontrol::Target {
            x: x.eval().eval(),
            y: y.eval().eval(),
            shoot: shoot.eval().eval(),
        };
        let command = command::ucontrol::Ucontrol::from(command);
        self.core.borrow_mut().push_command(command)
    }

    #[doc(alias = "targetp")]
    pub fn shootp<At: Target>(&self, at: impl Eval<At>, shoot: impl Eval<Number>) {
        assert_same_core!(self, at, shoot);
        let command = command::ucontrol::Targetp {
            unit: at.eval().eval(),
            shoot: shoot.eval().eval(),
        };
        let command = command::ucontrol::Ucontrol::from(command);
        self.core.borrow_mut().push_command(command)
    }
}
