use std::cell::RefCell;

use crate::{
    command::Command,
    eval::Eval,
    r#ref::Ref,
    types::{
        building::Building,
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

    pub fn new_unnamed<T: Type>(&self) -> Ref<'_, T> {
        let raw = &mut self.borrow_mut();
        let temp_name = raw.alloc_name();
        let raw = raw.new_variable(temp_name);
        self.make_ref(raw)
    }

    pub fn new_uninit<T: Type>(&self, name: impl Eval<String>) -> Ref<'_, T> {
        let raw = &mut self.borrow_mut();
        let var = raw.new_variable(name);
        self.make_ref(var)
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

    /// `@thisx` process variable
    pub fn thisx(&self) -> Ref<'_, Number> {
        self.make_ref(AT_THISX_IDX)
    }

    /// `@thisy` process variable
    pub fn thisy(&self) -> Ref<'_, Number> {
        self.make_ref(AT_THISY_IDX)
    }

    /// `@counter` process variable
    pub fn counter(&self) -> Ref<'_, Number> {
        self.make_ref(AT_COUNTER_IDX)
    }

    /// `@links` process variable
    pub fn links(&self) -> Ref<'_, Number> {
        self.make_ref(AT_LINKS_IDX)
    }

    /// `@ipt` process variable
    pub fn ipt(&self) -> Ref<'_, Number> {
        self.make_ref(AT_IPT_IDX)
    }

    /// `@time` process variable
    pub fn time(&self) -> Ref<'_, Number> {
        self.make_ref(AT_IPT_IDX)
    }

    /// `@tick` process variable
    pub fn tick(&self) -> Ref<'_, Number> {
        self.make_ref(AT_TICK_IDX)
    }

    /// `@mapw` process variable
    pub fn mapw(&self) -> Ref<'_, Number> {
        self.make_ref(AT_MAPW_IDX)
    }

    /// `@maph` process variable
    pub fn maph(&self) -> Ref<'_, Number> {
        self.make_ref(AT_MAPH_IDX)
    }

    // pub fn linkeds(&self) -> impl Iterator<Item = (Ref<'_, Number>, Ref<'_, Building>)> {
    //     let max = self.links();
    //     let mut counter = self.new_unnamed::<Number>();

    //     counter %= max;

    //     todo!()
    // }

    pub fn from_mdt<T: Type>(&self, name: impl Eval<String>) -> Ref<'_, T> {
        let name = name.eval();
        let idx = self.borrow_mut().new_variable(name);
        self.make_ref(idx)
    }

    pub fn generate(self) -> std::string::String {
        self.inner.borrow().generate()
    }

    pub fn write_to(self, mut target: impl std::io::Write) -> std::io::Result<()> {
        let code = self.inner.borrow().generate();
        target.write_all(code.as_bytes())?;
        target.flush()
    }

    pub fn write_to_stdout(self) {
        self.write_to(std::io::stdout()).unwrap();
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

    pub fn generate(&self) -> std::string::String {
        use std::fmt::Write;
        self.main
            .commands
            .iter()
            .fold(std::string::String::new(), |mut buffer, command| {
                buffer.write_fmt(format_args!("{command}\n")).ok();
                buffer
            })
    }
    pub(crate) fn context_variable(&self, idx: VariableIdx) -> Option<&String> {
        let var = match idx {
            AT_UNIT_IDX => &UNIT,
            AT_COUNT_IDX => &COUNT,
            AT_THISX_IDX => &THISX,
            AT_THISY_IDX => &THISY,
            AT_COUNTER_IDX => &COUNTER,
            AT_LINKS_IDX => &LINKS,
            AT_IPT_IDX => &IPT,
            AT_TICK_IDX => &TICK,
            AT_TIME_IDX => &TIME,
            AT_MAPW_IDX => &MAPW,
            AT_MAPH_IDX => &MAPH,
            _ => return None,
        };
        Some(var)
    }

    pub(crate) fn get_variable(&self, idx: VariableIdx) -> &String {
        match self.context_variable(idx) {
            Some(var) => var,
            None => &self.variables[idx.0],
        }
    }
}

/// max to `2^60-1` (on x64)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
pub(crate) const AT_LINKS_IDX: VariableIdx = VariableIdx(0b0110 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const LINKS: String = String::Static("@links");
pub(crate) const AT_IPT_IDX: VariableIdx = VariableIdx(0b0111 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const IPT: String = String::Static("@ipt");
pub(crate) const AT_TICK_IDX: VariableIdx = VariableIdx(0b1000 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const TICK: String = String::Static("@tick");
pub(crate) const AT_TIME_IDX: VariableIdx = VariableIdx(0b1001 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const TIME: String = String::Static("@time");
pub(crate) const AT_MAPW_IDX: VariableIdx = VariableIdx(0b1010 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const MAPW: String = String::Static("@mapw");
pub(crate) const AT_MAPH_IDX: VariableIdx = VariableIdx(0b1011 << (TARGET_POINTER_WIDTH - 4));
pub(crate) const MAPH: String = String::Static("@maph");

impl std::ops::Index<VariableIdx> for RawProcessor {
    type Output = String;

    fn index(&self, idx: VariableIdx) -> &Self::Output {
        self.get_variable(idx)
    }
}

#[derive(Default, Debug)]
pub struct Block {
    commands: Vec<Command>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let core = Processor::default();
        let mut a = core.new_uninit::<Number>("abcd");
        a += 1;

        core.write_to_stdout();
    }
}
