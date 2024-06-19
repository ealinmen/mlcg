use std::marker::PhantomData;

use crate::{
    command,
    eval::{Eval, WithCore},
    processor::{Processor, VariableIdx},
    types::Type,
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
        match ($crate::eval::WithCore::core(&$a), $crate::eval::WithCore::core(&$b)) {
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
    default fn eval(self) -> U {
        T::from_name(self.core.borrow()[self.idx].clone()).eval()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::unit::Unit;

    use super::*;

    #[test]
    fn shoot() {
        let core = Processor::default();
        let at_unit = core.unit();
        at_unit.target(core.thisx(), core.thisy(), true);

        let _awa = at_unit.save_as("awa");
        let awa = core.from_mdt::<Unit>("awa");
        awa.bind().target(core.thisx(), core.thisy(), true);

        core.write_to_stdout();
    }
}
