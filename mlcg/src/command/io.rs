use crate::String;

#[derive(Debug, Clone)]
pub struct Read {
    pub dst: String,
    pub from: String,
    pub at: usize,
}

#[derive(Debug, Clone)]
pub struct Write {
    pub src: String,
    pub to: String,
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
