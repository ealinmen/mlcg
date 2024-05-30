use std::cell::RefCell;

use crate::{command::Command, r#ref::Ref, types::Type, String};

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
        format!("v{}", idx).into()
    }

    pub(crate) fn new_variable(&mut self, name: impl Into<String>) -> VariableIdx {
        let name = name.into();
        self.variables.push(name);
        VariableIdx(self.variables.len() - 1)
    }

    pub(crate) fn push_command(&mut self, command: impl Into<Command>) {
        self.main.commands.push(command.into());
    }
}

#[derive(Clone, Copy)]
pub(crate) struct VariableIdx(usize);

impl From<usize> for VariableIdx {
    fn from(idx: usize) -> Self {
        VariableIdx(idx)
    }
}

impl std::ops::Index<VariableIdx> for RawProcessor {
    type Output = String;

    fn index(&self, index: VariableIdx) -> &Self::Output {
        &self.variables[index.0]
    }
}

#[derive(Default, Debug)]
pub struct Block {
    commands: Vec<Command>,
}
