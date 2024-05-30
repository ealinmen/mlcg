use crate::String;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Command {
    Read(Read),
    Write(Write),
    Draw(Draw),
    Print(Print),
    DrawFlush(DrawFlush),
    PrintFlush(PrintFlush),
    Operation(Operation),
}

mod froms {
    use super::*;
    impl From<Read> for Command {
        fn from(v: Read) -> Self {
            Self::Read(v)
        }
    }

    impl From<Write> for Command {
        fn from(v: Write) -> Self {
            Self::Write(v)
        }
    }

    impl From<Draw> for Command {
        fn from(v: Draw) -> Self {
            Self::Draw(v)
        }
    }

    impl From<Print> for Command {
        fn from(v: Print) -> Self {
            Self::Print(v)
        }
    }

    impl From<DrawFlush> for Command {
        fn from(v: DrawFlush) -> Self {
            Self::DrawFlush(v)
        }
    }

    impl From<PrintFlush> for Command {
        fn from(v: PrintFlush) -> Self {
            Self::PrintFlush(v)
        }
    }

    impl From<Operation> for Command {
        fn from(v: Operation) -> Self {
            Self::Operation(v)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Write {
    pub src: String,
    pub to: String,
    pub at: usize,
}

#[derive(Debug, Clone)]
pub struct Read {
    pub dst: String,
    pub from: String,
    pub at: usize,
}

#[derive(Debug, Clone)]
pub enum Draw {
    Clear {
        r: String,
        g: String,
        b: String,
    },
    ColorRGBA {
        r: String,
        g: String,
        b: String,
        a: String,
    },
    ColorHEX {
        color: String,
    },
    Stroke {
        width: String,
    },
    Line {
        x1: String,
        y1: String,
        x2: String,
        y2: String,
    },
    Rect {
        x: String,
        y: String,
        width: String,
        height: String,
    },
}

#[derive(Debug, Clone)]
pub struct Print {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct DrawFlush {
    pub to: String,
}

#[derive(Debug, Clone)]
pub struct PrintFlush {
    pub to: String,
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

pub struct Link {
    pub idx: usize,
    pub offset: usize,
}

pub enum Line {
    Link(Link),
    Line(usize),
}
