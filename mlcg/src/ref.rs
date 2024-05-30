use std::marker::PhantomData;

use crate::{
    eval::Eval,
    processor::{Processor, VariableIdx},
    types::Type,
};

pub struct Ref<'a, T>
where
    T: Type,
{
    pub(crate) core: &'a Processor,
    pub(crate) idx: VariableIdx,
    pub(crate) _type: PhantomData<T>,
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
