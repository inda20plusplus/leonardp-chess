use crate::color::Color;

#[derive(PartialEq, Debug, Clone)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

impl PieceKind {
    pub fn letter(&self) -> &str {
        self.ascii_meta().0
    }
    pub fn ascii(&self, color: Color) -> &str {
        match color {
            Color::White => self.ascii_meta().2,
            Color::Black => self.ascii_meta().1,
        }
    }
    pub fn value(&self) -> u32 {
        self.ascii_meta().3
    }
    pub fn ascii_meta(&self) -> (&str, &str, &str, u32) {
        match self {
            PieceKind::King => ("K", "♔", "♚", 9),
            PieceKind::Queen => ("Q", "♕", "♛", 9),
            PieceKind::Rook => ("R", "♖", "♜", 5),
            PieceKind::Knight => ("N", "♘", "♞", 3),
            PieceKind::Bishop => ("B", "♗", "♝", 3),
            PieceKind::Pawn => ("P", "♙", "♟︎", 1),
        }
    }
    fn from_letter(letter: &str) -> Option<PieceKind> {
        // TODO: auto gen from ascii_meta / keep DRY
        match letter {
            "K" => Option::Some(PieceKind::King),
            "Q" => Option::Some(PieceKind::Queen),
            "R" => Option::Some(PieceKind::Rook),
            "N" => Option::Some(PieceKind::Knight),
            "B" => Option::Some(PieceKind::Bishop),
            "P" => Option::Some(PieceKind::Pawn),
            _ => Option::None,
        }
    }
    pub fn from_str(source: &str) -> Option<PieceKind> {
        Self::from_letter(source)
    }
    pub fn jumps(&self) -> bool {
        match self {
            PieceKind::Knight => true,
            _ => false,
        }
    }
    pub fn delta_move_valid(&self, dx: i32, dy: i32) -> Result<(), &str> {
        let any_move = dx != 0 || dy != 0;
        let is_diagonal = i32::abs(dx) == i32::abs(dy);
        let is_vertical = dx == 0;
        let is_horizontal = dy == 0;
        let is_straight = is_vertical || is_horizontal;
        let max_one = i32::abs(dx) <= 1 && i32::abs(dy) <= 1;

        let ok = match self {
            PieceKind::King => any_move && (is_horizontal || max_one),
            PieceKind::Queen => any_move && (is_diagonal || is_straight),
            PieceKind::Rook => any_move && is_straight,
            PieceKind::Knight => match i32::abs(dy) {
                2 => i32::abs(dx) == 1,
                1 => i32::abs(dx) == 2,
                _ => false,
            },
            PieceKind::Bishop => any_move && is_diagonal,
            PieceKind::Pawn => any_move && ((is_diagonal && max_one) || is_vertical),
        };
        match ok {
            true => Ok(()),
            false => Result::Err("invalid move"),
        }
    }
    pub fn delta_steps(&self, dx: i32, dy: i32) -> Vec<(i32, i32)> {
        // TODO: optimisation(minor): convertable to iterator,
        //  thus only generating as "tiles are explored"
        let dxs: Vec<i32> = (0..i32::abs(dx))
            .map(|_| if dx < 0 { -1 } else { 1 })
            .collect();
        let dys: Vec<i32> = (0..i32::abs(dy))
            .map(|_| if dy < 0 { -1 } else { 1 })
            .collect();

        (0..usize::max(dxs.len(), dys.len()))
            .map(|i| {
                let dy = *dys.get(i).unwrap_or(&0);
                let dx = *dxs.get(i).unwrap_or(&0);
                (dx, dy)
            })
            .collect()
    }
}
