use std::marker::PhantomData;

use crate::{
    abilities::Shoot,
    command::Set,
    eval::Eval,
    processor::{Processor, VariableIdx},
    types::{building::Buildings, number::Number, unit::Units, Type},
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
}

impl<'a, T> Ref<'a, T>
where
    T: Type + Shoot + Units,
{
    #[doc(alias = "shoot")]
    pub fn target(&self, x: impl Eval<Number>, y: impl Eval<Number>, shoot: impl Eval<Number>) {
        todo!()
    }

    #[doc(alias = "shootp")]
    pub fn targetp(&self, target: impl Eval<()>, shoot: impl Eval<Number>) {
        todo!()
    }
}

impl<'a, T> Ref<'a, T>
where
    T: Type + Shoot + Buildings,
{
    #[doc(alias = "target")]
    pub fn shoot(&self, x: impl Eval<Number>, y: impl Eval<Number>, shoot: impl Eval<Number>) {
        todo!()
    }

    #[doc(alias = "targetp")]
    pub fn shootp(&self, target: impl Eval<()>, shoot: impl Eval<Number>) {
        todo!()
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

impl<'a, T, U> Eval<U> for Ref<'a, T>
where
    T: Eval<U> + Type,
{
    fn eval(self) -> U {
        T::from_name(self.core.borrow()[self.idx].clone()).eval()
    }
}
