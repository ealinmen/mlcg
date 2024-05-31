mod control;
mod io;
mod operate;
mod unit;
pub use control::*;
pub use io::*;
pub use operate::*;
pub use unit::*;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Command {
    Read(Read),
    Write(Write),
    Draw(Draw),
    Print(Print),

    DrawFlush(DrawFlush),
    PrintFlush(PrintFlush),
    GetLink(GetLink),

    Set(Set),
    Operation(Operation),

    UnitBind(UnitBind),
    UnitControl(UnitControl),
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

    impl From<GetLink> for Command {
        fn from(v: GetLink) -> Self {
            Self::GetLink(v)
        }
    }

    impl From<Set> for Command {
        fn from(v: Set) -> Self {
            Self::Set(v)
        }
    }

    impl From<Operation> for Command {
        fn from(v: Operation) -> Self {
            Self::Operation(v)
        }
    }

    impl From<UnitBind> for Command {
        fn from(v: UnitBind) -> Self {
            Self::UnitBind(v)
        }
    }

    impl From<UnitControl> for Command {
        fn from(v: UnitControl) -> Self {
            Self::UnitControl(v)
        }
    }
}

pub struct Link {
    pub idx: usize,
    pub offset: usize,
}

pub enum Line {
    Link(Link),
    Line(usize),
}
