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

#[derive(Debug, Clone)]
pub enum Control {
    Enable {
        of: String,
        enable: String,
    },
    Shoot {
        of: String,
        x: String,
        y: String,
        shoot: String,
    },
    Shootp {
        of: String,
        at: String,
        shoot: String,
    },
    Config {
        of: String,
        to: String,
    },
    Color {
        of: String,
        to: String,
    },
}
// control enabled block1 0 0 0 0
impl std::fmt::Display for Control {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("control ")?;
        match self {
            Control::Enable { of, enable } => write!(f, "enabled {} {} 0 0 0", of, enable),
            Control::Shoot { of, x, y, shoot } => write!(f, "shoot {} {} {} {} 0", of, x, y, shoot),
            Control::Shootp { of, at, shoot } => write!(f, "shootp {} {} {} 0 0", of, at, shoot),
            Control::Config { of, to } => write!(f, "config {} {} 0 0 0 ", of, to),
            Control::Color { of, to } => write!(f, "color {} {} 0 0 0 ", of, to),
        }
    }
}
