use crate::{eval::Eval, r#ref::Ref, String};

use super::Type;

#[derive(Eval)]
pub enum Number {
    Immediate(f64),
    Variable(String),
}

impl Number {
    fn remake(self) -> Self {
        if let Self::Variable(name) = &self {
            if let Ok(f) = name.parse::<f64>() {
                return Self::Immediate(f);
            }
        }
        self
    }
}

impl Type for Number {
    fn from_name(name: String) -> Self {
        match name.parse::<f64>() {
            Ok(f) => Self::Immediate(f),
            Err(_) => Self::Variable(name),
        }
    }
}

impl<F> Eval<Number> for F
where
    f64: From<F>,
{
    fn eval(self) -> Number {
        Number::Immediate(self.into())
    }
}

macro_rules! binary_ops_impl {
    ($($trait:ident => $method:ident,)*) => {
        $(
        impl<N> std::ops::$trait<N> for Ref<'_, Number>
        where
            N: Eval<Number>,
        {
            type Output = Self;

            fn $method(self, rhs: N) -> Self::Output {
                use crate::command::*;
                let processor = self.core;

                let n1 = self.eval();
                let n2 = rhs.eval();
                let (l, r) = match (n1.remake(), n2.remake()) {
                    (Number::Immediate(a), Number::Immediate(b)) => {
                        return processor.make_ref(
                            processor
                                .borrow_mut()
                                .new_variable(a.$method(b).to_string()),
                        )
                    }
                    (Number::Immediate(l), Number::Variable(r)) => (crate::String::new(l), r),
                    (Number::Variable(l), Number::Immediate(r)) => (l, crate::String::new(r)),
                    (Number::Variable(l), Number::Variable(r)) => (l, r),
                };
                let result = {
                    let mut processor = processor.borrow_mut();
                    let result = processor.alloc_name();
                    processor.push_command(Operation::Binary {
                        op: Op::$trait,
                        result: result.clone(),
                        lhs: l,
                        rhs: r,
                    });
                    processor.new_variable(result)
                };
                processor.make_ref(result)
            }
        }
        )*
    };
}

binary_ops_impl! {
    Add => add,
    Sub => sub,
    Mul => mul,
    Div => div,
    Rem => rem,
}

#[cfg(test)]
mod tests {
    use crate::processor::Processor;

    use super::*;

    #[test]
    fn operator() {
        let core = Processor::default();
        let a: Ref<'_, Number> = core.make_ref(core.borrow_mut().new_variable("a"));
        let b: Ref<'_, Number> = core.make_ref(core.borrow_mut().new_variable("b"));

        let c = a + b;
        let d = c + 114514;
        let e = d + false;
        let _f = e + 1919.810;

        const EXPECTED: &str = r#"Block { commands: [Operation(Binary { op: Add, result: Rc("v0"), lhs: Static("a"), rhs: Static("b") }), Operation(Binary { op: Add, result: Rc("v1"), lhs: Rc("v0"), rhs: Rc("114514") }), Operation(Binary { op: Add, result: Rc("v2"), lhs: Rc("v1"), rhs: Rc("0") }), Operation(Binary { op: Add, result: Rc("v3"), lhs: Rc("v2"), rhs: Rc("1919.81") })] }"#;
        assert_eq!(format!("{:?}", core.borrow().main), EXPECTED,);
    }
}
