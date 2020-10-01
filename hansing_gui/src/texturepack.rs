use ggez::graphics::*;
use ggez::Context;

pub struct Texturepack {
    pub piece_texture: Vec<Image>,
    pub placeholder: bool,
}
impl Texturepack {
    pub fn new_placeholder() -> Self {
        Self {
            piece_texture: Vec::new(),
            placeholder: true,
        }
    }
    pub fn new() -> Self {
        Self {
            piece_texture: Vec::new(),
            placeholder: false,
        }
    }
    pub fn texture_from_index(&self, index: usize) -> Option<Image> {
        if index >= self.piece_texture.len() {
            None
        } else {
            Some(self.piece_texture[index].clone())
        }
    }
}

const PIECE_NAMES: [&str; 6] = ["pawn", "knight", "bishop", "rook", "queen", "king"];

pub fn make_texturepack(c: &mut Context) -> Texturepack {
    let mut tp = Texturepack::new();
    for color in ["w", "b"].iter() {
        for piece in PIECE_NAMES.iter() {
            tp.piece_texture
                .push(Image::new(c, format!("/{}_{}.png", color, piece)).unwrap());
        }
    }
    tp
}
