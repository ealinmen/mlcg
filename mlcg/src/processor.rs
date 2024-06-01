use std::cell::RefCell;

use crate::{
    command::Command,
    eval::Eval,
    r#ref::Ref,
    types::{
        number::Number,
        unit::{Unit, Units},
        Type,
    },
    String,
};

#[derive(Default)]
pub struct Processor {
    pub(crate) inner: RefCell<RawProcessor>,
}

impl Processor {
    pub(crate) fn borrow(&self) -> std::cell::Ref<'_, RawProcessor> {
        self.inner.borrow()
    }

    pub(crate) fn borrow_mut(&self) -> std::cell::RefMut<'_, RawProcessor> {
        self.inner.borrow_mut()
    }

    pub(crate) fn make_ref<T: Type>(&self, idx: VariableIdx) -> Ref<'_, T> {
        Ref {
            core: self,
            idx,
            _type: Default::default(),
        }
    }

    pub(crate) fn is_same_core(&self, rhs: &Self) -> bool {
        (self as *const Processor).eq(&(rhs as _))
    }

    pub fn unit_bind<U: Units>(&self) {
        self.borrow_mut()
            .push_command(crate::command::ubind::Ubind {
                ty: U::class_name().eval(),
            });
    }

    /// # Note
    ///
    /// the `@unit` may is not binded yet
    pub fn unit(&self) -> Ref<'_, Unit> {
        self.make_ref(AT_UNIT_IDX)
    }

    /// `@thisx` context variable
    pub fn thisx(&self) -> Ref<'_, Number> {
        self.make_ref(AT_THISX_IDX)
    }

    /// `@thisy` context variable
    pub fn thisy(&self) -> Ref<'_, Number> {
        self.make_ref(AT_THISY_IDX)
    }

    pub fn counter(&self) -> Ref<'_, Number> {
        self.make_ref(AT_COUNTER_IDX)
    }

    pub fn from_mdt<T: Type>(&self, name: impl Eval<String>) -> Ref<'_, T> {
        let name = name.eval();
        let idx = self.borrow_mut().new_variable(name);
        self.make_ref(idx)
    }
}

#[derive(Default, Debug)]
pub(crate) struct RawProcessor {
    pub main: Block,
    pub appends: Vec<Block>,
    pub variables: Vec<String>,
    pub alloc: usize,
}

impl RawProcessor {
    pub(crate) fn alloc_name(&mut self) -> String {
        let idx = self.alloc;
        self.alloc += 1;
        format!("v{}", idx).eval()
    }

    pub(crate) fn new_variable(&mut self, name: impl Eval<String>) -> VariableIdx {
        let name = name.eval();
        self.variables.push(name);
        VariableIdx(self.variables.len() - 1)
    }

    pub(crate) fn push_command(&mut self, command: impl Into<Command>) {
        self.main.commands.push(command.into());
    }
}

/// max to `2^60-1` (on x64)
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VariableIdx(usize);

impl From<usize> for VariableIdx {
    fn from(idx: usize) -> Self {
        VariableIdx(idx)
    }
}

#[cfg(target_pointer_width = "64")]
const TARGET_POINTER_WIDTH: usize = 64;
#[cfg(target_pointer_width = "32")]
const TARGET_POINTER_WIDTH: usize = 32;

// there are some magic_number: @unit @counter @thisx @thisy @this...
// 4 bit is kept for them
pub(crate) const AT_UNIT_IDX: VariableIdx = VariableIdx(0b0001 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const UNIT: String = String::Static("@unit");
pub(crate) const AT_COUNT_IDX: VariableIdx = VariableIdx(0b0010 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const COUNT: String = String::Static("@count");
pub(crate) const AT_THISX_IDX: VariableIdx = VariableIdx(0b0011 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const THISX: String = String::Static("@thisx");
pub(crate) const AT_THISY_IDX: VariableIdx = VariableIdx(0b0100 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const THISY: String = String::Static("@thisy");
pub(crate) const AT_COUNTER_IDX: VariableIdx = VariableIdx(0b0101 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const COUNTER: String = String::Static("@counter");

impl std::ops::Index<VariableIdx> for RawProcessor {
    type Output = String;

    fn index(&self, index: VariableIdx) -> &Self::Output {
        match index {
            AT_UNIT_IDX => &UNIT,
            AT_COUNT_IDX => &COUNT,
            AT_THISX_IDX => &THISX,
            AT_THISY_IDX => &THISY,
            AT_COUNTER_IDX => &COUNTER,
            _ => &self.variables[index.0],
        }
    }
}

#[derive(Default, Debug)]
pub struct Block {
    commands: Vec<Command>,
}
