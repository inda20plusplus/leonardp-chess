#[derive(Copy, Clone)]
pub enum PrintStyle {
    Ascii,
}

#[derive(Copy, Clone)]
pub struct BoardPrintStyle {
    pub style: PrintStyle,
    pub border: bool,
    pub number: bool,
}

impl BoardPrintStyle {
    pub fn ascii_pretty() -> BoardPrintStyle {
        BoardPrintStyle {
            style: PrintStyle::Ascii,
            border: true,
            number: true,
        }
    }
    pub fn ascii_bordered() -> BoardPrintStyle {
        BoardPrintStyle {
            style: PrintStyle::Ascii,
            border: true,
            number: false,
        }
    }
}
