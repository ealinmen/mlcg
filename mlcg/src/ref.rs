use std::marker::PhantomData;

use crate::{
    abilities::{Shoot, Target},
    command,
    eval::{Eval, WithCore},
    processor::{Processor, VariableIdx},
    types::{
        building::Buildings,
        number::Number,
        unit::{Unit, Units},
        Type,
    },
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
        let set = command::set::Set {
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
        let command = command::set::Set {
            result: self.core.borrow()[self.idx].clone(),
            value: value.eval().eval(),
        };
        self.core.borrow_mut().push_command(command)
    }

    pub fn cast<T2: Type>(self) -> Ref<'a, T2> {
        Ref {
            core: self.core,
            idx: self.idx,
            _type: PhantomData,
        }
    }
}

impl<'a, U: Units> Ref<'a, Unit<U>> {
    pub fn bind(&self) -> Ref<'a, Unit> {
        let s = self.cast::<Unit>();
        self.core.unit().set_to(s);
        s
    }
}

impl<'a> Ref<'a, Unit> {
    #[doc(alias = "shoot")]
    pub fn target(&self, x: impl Eval<Number>, y: impl Eval<Number>, shoot: impl Eval<Number>) {
        assert_same_core!(self, x, y, shoot);
        let command = command::ucontrol::Target {
            x: x.eval().eval(),
            y: y.eval().eval(),
            shoot: shoot.eval().eval(),
        };
        let command = command::ucontrol::Ucontrol::from(command);
        self.core.borrow_mut().push_command(command);
    }

    #[doc(alias = "shootp")]
    pub fn targetp<At: Target>(&self, at: impl Eval<At>, shoot: impl Eval<Number>) {
        assert_same_core!(self, at, shoot);
        let command = command::ucontrol::Targetp {
            unit: at.eval().eval(),
            shoot: shoot.eval().eval(),
        };
        let command = command::ucontrol::Ucontrol::from(command);
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

        let _awa = at_unit.save_as("awa");
        let awa = processor.from_mdt::<Unit>("awa");
        awa.bind()
            .target(processor.thisx(), processor.thisy(), true);

        const EXPECTD: &str = r#"Block { commands: [UnitControl(Target { x: Static("@thisx"), y: Static("@thisy"), shoot: Rc("1") }), Set(Set { result: Static("awa"), value: Static("@unit") }), Set(Set { result: Static("@unit"), value: Static("awa") }), UnitControl(Target { x: Static("@thisx"), y: Static("@thisy"), shoot: Rc("1") })] }"#;
        assert_eq!(format!("{:?}", processor.borrow().main), EXPECTD);
    }
}
