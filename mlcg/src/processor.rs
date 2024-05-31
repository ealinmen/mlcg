use std::cell::RefCell;

use crate::{command::Command, eval::Eval, r#ref::Ref, types::Type, String};

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

// there are some magic_number: @unit @counter @thisx @thisy @self...
// 4 bit is kept for them
pub(crate) const AT_UNIT_IDX: VariableIdx = VariableIdx(0b0000 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const UNIT: String = String::Static("@unit");
pub(crate) const AT_COUNT_IDX: VariableIdx = VariableIdx(0b0001 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const COUNT: String = String::Static("@count");

impl std::ops::Index<VariableIdx> for RawProcessor {
    type Output = String;

    fn index(&self, index: VariableIdx) -> &Self::Output {
        match index {
            AT_UNIT_IDX => &UNIT,
            AT_COUNT_IDX => &COUNT,
            _ => &self.variables[index.0],
        }
    }
}

#[derive(Default, Debug)]
pub struct Block {
    commands: Vec<Command>,
}
