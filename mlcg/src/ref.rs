use std::marker::PhantomData;

use crate::{
    abilities::{Shoot, Target},
    command::{Control, Set, UnitControl},
    eval::{Eval, WithCore},
    processor::{Processor, VariableIdx},
    types::{building::Buildings, number::Number, unit::Unit, Type},
    String,
};

pub struct Ref<'a, T>
where
    T: Type,
{
    pub(crate) core: &'a Processor,
    pub(crate) idx: VariableIdx,
    pub(crate) _type: PhantomData<T>,
}

macro_rules! assert_same_core {
    ($($refs:tt),*) => {
        assert_same_core!(@rec $($refs),*)
    };
    (@rec $a:tt) => {};
    (@rec $a:tt, $b:tt $(,$rest:tt)*) => {
        match ($a.core(), $b.core()) {
            (Some(a_core), Some(b_core)) => assert!(a_core.is_same_core(b_core), "refernces must be in same processor!"),
            _ => {}
        }
        assert_same_core!(@rec $b $(,$rest)*)
    }
}

impl<'a, T> Ref<'a, T>
where
    T: Type,
{
    #[doc(alias = "set")]
    pub fn save_as(&self, name: impl Eval<String>) -> Self {
        let result = {
            let name = name.eval();
            if name.is_empty() {
                self.core.borrow_mut().alloc_name()
            } else {
                name
            }
        };
        let set = Set {
            result: result.clone(),
            value: self.core.borrow()[self.idx].clone(),
        };
        let result = {
            let mut core = self.core.borrow_mut();
            core.push_command(set);
            core.new_variable(result)
        };
        self.core.make_ref(result)
    }

    pub fn set_to(&self, value: impl Eval<T>) {
        let command = Set {
            result: self.core.borrow()[self.idx].clone(),
            value: value.eval().eval(),
        };
        self.core.borrow_mut().push_command(command)
    }
}

impl<'a> Ref<'a, Unit> {
    #[doc(alias = "shoot")]
    pub fn target(&self, x: impl Eval<Number>, y: impl Eval<Number>, shoot: impl Eval<Number>) {
        assert_same_core!(self, x, y, shoot);
        let command = UnitControl::Target {
            x: x.eval().eval(),
            y: y.eval().eval(),
            shoot: shoot.eval().eval(),
        };
        self.core.borrow_mut().push_command(command);
    }

    #[doc(alias = "shootp")]
    pub fn targetp<At: Target>(&self, at: impl Eval<At>, shoot: impl Eval<Number>) {
        assert_same_core!(self, at, shoot);
        let command = UnitControl::Targetp {
            at: at.eval().eval(),
            shoot: shoot.eval().eval(),
        };
        self.core.borrow_mut().push_command(command)
    }
}

impl<'a, T> Ref<'a, T>
where
    T: Type + Shoot + Buildings,
{
    #[doc(alias = "target")]
    pub fn shoot(&self, x: impl Eval<Number>, y: impl Eval<Number>, shoot: impl Eval<Number>) {
        assert_same_core!(self, x, y, shoot);
        let command = Control::Shoot {
            of: (*self).eval(),
            x: x.eval().eval(),
            y: y.eval().eval(),
            shoot: shoot.eval().eval(),
        };
        self.core.borrow_mut().push_command(command)
    }

    #[doc(alias = "targetp")]
    pub fn shootp<At: Target>(&self, at: impl Eval<At>, shoot: impl Eval<Number>) {
        assert_same_core!(self, at, shoot);
        let command = Control::Shootp {
            of: (*self).eval(),
            at: at.eval().eval(),
            shoot: shoot.eval().eval(),
        };
        self.core.borrow_mut().push_command(command)
    }
}

impl<'a, T> Clone for Ref<'a, T>
where
    T: Type,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for Ref<'a, T> where T: Type {}

impl<'a, T: Type> WithCore for Ref<'a, T> {
    fn core(&self) -> Option<&Processor> {
        Some(self.core)
    }
}

impl<'a, T, U> Eval<U> for Ref<'a, T>
where
    T: Eval<U> + Type,
{
    fn eval(self) -> U {
        T::from_name(self.core.borrow()[self.idx].clone()).eval()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shoot() {
        let processor = Processor::default();
        let at_unit = processor.unit();
        at_unit.target(processor.thisx(), processor.thisy(), true);

        at_unit.save_as("awa");

        let awa = processor.from_mdt::<Unit>("awa");

        awa.set_to(at_unit);

        dbg!(&processor.borrow().main);
    }
}
