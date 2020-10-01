//use crate::assets;
use ggez::graphics::*;
use ggez::Context;
//#[derive(Clone)]
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
    /*pub fn texture_from_char(&self, ch: char) -> Option<Image> {
        self.texture_from_index(match ch {
            'P' => 1,
            'N' => 2,
            'B' => 3,
            'R' => 4,
            'Q' => 5,
            'K' => 6,
            'p' => 7,
            'n' => 8,
            'b' => 9,
            'r' => 10,
            'q' => 11,
            'k' => 12,
            _ => 0,
        })
    }*/
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

/*
pub fn make_texture_pack(c: &mut Context) -> Texturepack {
    let mut tp = Texturepack::new();
    let w_pawn = assets::piece_asset_from_char(c, 'P').unwrap();
    tp.piece_texture.push(w_pawn);
    let w_knight = assets::piece_asset_from_char(c, 'N').unwrap();
    tp.piece_texture.push(w_knight);
    let w_bishop = assets::piece_asset_from_char(c, 'B').unwrap();
    tp.piece_texture.push(w_bishop);
    let w_rook = assets::piece_asset_from_char(c, 'R').unwrap();
    tp.piece_texture.push(w_rook);
    let w_queen = assets::piece_asset_from_char(c, 'Q').unwrap();
    tp.piece_texture.push(w_queen);
    let w_king = assets::piece_asset_from_char(c, 'K').unwrap();
    tp.piece_texture.push(w_king);
    let b_pawn = assets::piece_asset_from_char(c, 'p').unwrap();
    tp.piece_texture.push(b_pawn);
    let b_knight = assets::piece_asset_from_char(c, 'n').unwrap();
    tp.piece_texture.push(b_knight);
    let b_bishop = assets::piece_asset_from_char(c, 'b').unwrap();
    tp.piece_texture.push(b_bishop);
    let b_rook = assets::piece_asset_from_char(c, 'r').unwrap();
    tp.piece_texture.push(b_rook);
    let b_queen = assets::piece_asset_from_char(c, 'q').unwrap();
    tp.piece_texture.push(b_queen);
    let b_king = assets::piece_asset_from_char(c, 'k').unwrap();
    tp.piece_texture.push(b_king);
    tp.initialized = true;
    tp
}*/
