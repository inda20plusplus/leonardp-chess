pub use crate::view::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    // (0, 0) is bottom left for white
    // TODO: consider using i32 instead?
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct File {
    pub n: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Rank {
    pub n: usize,
}

impl Position {
    pub fn from_str(source: &str) -> Option<Position> {
        let mut chars = vec![];

        source.to_uppercase().chars().for_each(|a| chars.push(a));
        if chars.len() != 2 {
            return Option::None;
        }

        let (x, y) = (chars[0] as u8, chars[1] as u8);
        if x < b'A' || x > b'Z' {
            return Option::None;
        }
        if y < b'0' || y > b'9' {
            return Option::None;
        }

        let x = x - b'A';
        let y = y - b'1';

        Option::Some(Position {
            x: x as usize,
            y: y as usize,
        })
    }
    pub fn to_string_code(&self) -> String {
        let file = File::new(self.x).print(PrintStyle::Ascii);
        format!("{}{}", file, self.y + 1)
    }
}

impl File {
    pub fn new(n: usize) -> File {
        File { n }
    }
    pub fn print(&self, style: PrintStyle) -> String {
        assert!(self.n < (b'Z' - b'A') as usize);
        match style {
            PrintStyle::Ascii => String::from_utf8_lossy(&[(self.n as u8) + b'A'])
                .to_owned()
                .to_string(),
        }
    }
}
