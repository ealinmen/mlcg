use crate::{
    eval::Eval,
    processor::{Processor, AT_UNIT_IDX},
    r#ref::Ref,
    types::unit::{Unit, Units},
    String,
};

#[derive(Debug, Clone)]
pub struct UnitBind {
    pub ty: String,
}

impl Processor {
    pub fn unit_bind<U: Units>(&self) {
        self.borrow_mut().push_command(UnitBind {
            ty: U::class_name().eval(),
        });
    }

    pub fn unit<U: Units>(&self) -> Ref<'_, Unit<U>> {
        self.make_ref(AT_UNIT_IDX)
    }
}

impl std::fmt::Display for UnitBind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ubind @ty
        write!(f, "ubind {}", self.ty)
    }
}

#[derive(Debug, Clone)]
pub enum UnitControl {
    Idle,
    Stop,
    Move {
        x: String,
        y: String,
    },
    Approach {
        x: String,
        y: String,
        radius: String,
    },
}
