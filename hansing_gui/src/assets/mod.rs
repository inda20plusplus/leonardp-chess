/*use ggez::graphics::Image;
use ggez::Context;
use std::path;

// READ AS: Color - - - PT PT PT PT
pub fn piece_asset_from_byte(c: &mut Context, b: u8) -> Option<Image> {
    if b == 0 {
        return None;
    }

    let path = if b & 0b1000_0000 == 0b0000_0000 {
        match b & 0b0000_1111 {
            1 => "/w_pawn.png",
            2 => "/w_knight.png",
            3 => "/w_bishop.png",
            4 => "/w_rook.png",
            5 => "/w_queen.png",
            6 => "/w_king.png",
            _ => "/missingno.png",
        }
    } else {
        match b & 0b0000_1111 {
            1 => "/b_pawn.png",
            2 => "/b_knight.png",
            3 => "/b_bishop.png",
            4 => "/b_rook.png",
            5 => "/b_queen.png",
            6 => "/b_king.png",
            _ => "/missingno.png",
        }
    };
    if let Ok(texture) = Image::new(c, path) {
        Some(texture)
    } else {
        None
    }
}
pub fn piece_asset_from_char(c: &mut Context, ch: char) -> Option<Image> {
    if ch == ' ' {
        return None;
    }
    let path = match ch {
        'P' => "/w_pawn.png",
        'N' => "/w_knight.png",
        'B' => "/w_bishop.png",
        'R' => "/w_rook.png",
        'Q' => "/w_queen.png",
        'K' => "/w_king.png",
        'p' => "/b_pawn.png",
        'n' => "/b_knight.png",
        'b' => "/b_bishop.png",
        'r' => "/b_rook.png",
        'q' => "/b_queen.png",
        'k' => "/b_king.png",
        _ => "/missingno.png",
    };
    println!("{}", path);
    if let Ok(texture) = Image::new(c, path) {
        Some(texture)
    } else {
        println!("{} is not a piece char", ch);
        None
    }
}
pub fn piece_asset_from_ascii(c: &mut Context, ascii: &str) -> Option<Image> {
    if ascii == "" || ascii == " " {
        return None;
    }
    let path = match ascii {
        "♙" => "/w_pawn.png",
        "♘" => "/w_knight.png",
        "♗" => "/w_bishop.png",
        "♖" => "/w_rook.png",
        "♕" => "/w_queen.png",
        "♔" => "/w_king.png",
        "♟︎" => "/b_pawn.png",
        "♞" => "/b_knight.png",
        "♝" => "/b_bishop.png",
        "♜" => "/b_rook.png",
        "♛" => "/b_queen.png",
        "♚" => "/b_king.png",
        _ => "/missingno.png",
    };
    if let Ok(texture) = Image::new(c, path) {
        Some(texture)
    } else {
        None
    }
}
*/