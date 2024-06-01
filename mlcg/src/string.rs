use std::rc::Rc;

use crate::eval::Eval;

#[derive(Eval, Debug, Clone)]
pub enum String {
    Rc(Rc<std::string::String>),
    Static(&'static str),
}

impl String {
    pub fn new(s: impl ToString) -> Self {
        Self::Rc(s.to_string().into())
    }
}

impl<S: ToString> Eval<String> for S {
    default fn eval(self) -> String {
        String::Rc(self.to_string().into())
    }
}

impl Eval<String> for &'static str {
    fn eval(self) -> String {
        String::Static(self)
    }
}

impl Eval<String> for std::string::String {
    fn eval(self) -> String {
        String::Rc(self.into())
    }
}

impl Eval<String> for Rc<std::string::String> {
    fn eval(self) -> String {
        String::Rc(self)
    }
}

impl Eval<String> for std::borrow::Cow<'_, str> {
    fn eval(self) -> String {
        match self {
            std::borrow::Cow::Borrowed(s) => s.to_string(),
            std::borrow::Cow::Owned(s) => s,
        }
        .eval()
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
