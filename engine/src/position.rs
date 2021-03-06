pub use crate::view::*;
use std::str::FromStr;

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
    pub fn to_string_code(&self) -> String {
        let file = File::new(self.x).print(PrintStyle::Ascii);
        format!("{}{}", file, self.y + 1)
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut chars = vec![];

        source.to_uppercase().chars().for_each(|a| chars.push(a));
        if chars.len() != 2 {
            return Err(());
        }

        let (x, y) = (chars[0] as u8, chars[1] as u8);
        if x < b'A' || x > b'Z' {
            return Err(());
        }
        if y < b'0' || y > b'9' {
            return Err(());
        }

        let x = x - b'A';
        let y = y - b'1';

        Ok(Position {
            x: x as usize,
            y: y as usize,
        })
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
