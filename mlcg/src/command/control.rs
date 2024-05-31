use crate::String;

#[derive(Debug, Clone)]
pub struct DrawFlush {
    pub to: String,
}

#[derive(Debug, Clone)]
pub struct PrintFlush {
    pub to: String,
}

#[derive(Debug, Clone)]
pub struct GetLink {
    pub result: String,
    /// [`usize`]
    pub link_to: String,
}

impl std::fmt::Display for GetLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "getlink {} {}", self.result, self.link_to)
    }
}
