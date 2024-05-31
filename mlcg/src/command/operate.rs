use crate::String;

#[derive(Debug, Clone)]
pub struct Set {
    pub result: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Binary {
        op: Op,
        result: String,
        lhs: String,
        rhs: String,
    },
    Unary {
        op: Op,
        result: String,
        value: String,
    },
}

/*
op add result a b
op sin result a b
*/
impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary {
                op,
                result,
                lhs,
                rhs,
            } => {
                write!(f, "op {} {} {} {}", op, result, lhs, rhs)
            }
            Self::Unary { op, result, value } => {
                write!(f, "op {} {} {}", op, result, value)
            }
        }
    }
}

macro_rules! ops {
    ($($name:ident => $value:literal,)*) => {
        #[derive(Debug,Clone)]
        pub enum Op {
            $($name,)*
        }

        impl std::fmt::Display for Op {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$name => write!(f, "{}", $value),)*
                }
            }
        }
    };
}

ops! {
    Add => "add",
    Sub => "sub",
    Mul => "mul",
    Div => "div",
    Rem => "rem",
    IDiv => "idiv",
}
