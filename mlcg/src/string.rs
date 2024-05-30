use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum String {
    Rc(Rc<std::string::String>),
    Static(&'static str),
}

impl String {
    pub fn new(s: impl ToString) -> Self {
        Self::Rc(s.to_string().into())
    }
}

impl From<&'static str> for String {
    fn from(v: &'static str) -> Self {
        Self::Static(v)
    }
}

impl From<std::string::String> for String {
    fn from(value: std::string::String) -> Self {
        Self::Rc(value.into())
    }
}

impl From<Rc<std::string::String>> for String {
    fn from(v: Rc<std::string::String>) -> Self {
        Self::Rc(v)
    }
}

impl std::ops::Deref for String {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Rc(s) => s.as_str(),
            Self::Static(s) => s,
        }
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rc(s) => write!(f, "{}", s),
            Self::Static(s) => write!(f, "{}", s),
        }
    }
}
