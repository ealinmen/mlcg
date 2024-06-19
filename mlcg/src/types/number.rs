use crate::{eval::Eval, r#ref::Ref, String};

use super::Type;

#[derive(Eval)]
pub enum Number {
    Immediate(f64),
    Variable(String),
}

impl Type for Number {
    fn from_name(name: String) -> Self {
        Self::Variable(name)
    }
}

impl Eval<String> for Number {
    fn eval(self) -> String {
        match self {
            Number::Immediate(f) => String::new(f),
            Number::Variable(s) => s,
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
                let result = processor.new_unnamed();
                let lhs: Number = self.eval();
                let rhs: Number = rhs.eval();
                {
                    let lhs: String = lhs.eval();
                    let rhs: String = rhs.eval();
                    let result = result.eval();
                    processor.borrow_mut().push_command(op::Op {
                        op: stringify!($method).eval(),
                        result,
                        lhs,
                        rhs,
                    });
                }
                result
            }
        }
        )*
    };
    ($($trait:ident($op:ident) => $method:ident($op_method:ident),)*) => {
        $(
        impl<N> std::ops::$trait<N> for Ref<'_, Number>
        where
            N: Eval<Number>,
        {
            fn $method(&mut self, rhs: N){
                use crate::command::*;
                let processor = self.core;

                let lhs: Number = self.eval();
                let lhs: String = lhs.eval();
                let rhs: Number = rhs.eval();

                let mut processor = processor.borrow_mut();
                processor.push_command(op::Op {
                    op: stringify!($op_method).eval(),
                    result: lhs.clone(),
                    lhs: lhs.eval(),
                    rhs: rhs.eval(),
                });
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

binary_ops_impl! {
    AddAssign(Add) => add_assign(add),
    SubAssign(Sub) => sub_assign(sub),
    MulAssign(Mul) => mul_assign(mul),
    DivAssign(Div) => div_assign(div),
    RemAssign(Rem) => rem_assign(rem),
}

#[cfg(test)]
mod tests {
    use crate::processor::Processor;

    use super::*;

    #[test]
    fn operator() {
        let core = Processor::default();
        let a = core.from_mdt::<Number>("a");
        let b = core.from_mdt::<Number>("b");

        let c = a + b;
        let d = c + 114514;
        let e = d + false;
        let _f = e + 1919.810;

        core.write_to_stdout();
    }

    #[test]
    fn operator_assign() {
        let core = Processor::default();
        let mut a = core.from_mdt("a");
        let mut b = core.from_mdt("b");
        a += b;
        b += a;
        let _c = a + b;

        core.write_to_stdout();
    }
}
