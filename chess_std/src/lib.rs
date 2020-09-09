#![allow(dead_code)]

// TODO: moves
// TODO: use mod + pub
// TODO: use test (unit)
// TODO: use test (full GM replay using PGN)

pub struct Game {
    pub board: Board,
    state: State,
    players: [Player; 2],
}

enum State {
    Active,
    Ended(StateEnded),
}

struct Player {
    color: Color,
    captured: Vec<Piece>,
}

#[derive(Debug, Copy, Clone)]
enum Color {
    White,
    Black,
}

// TODO: use refs instead?
type PlayerIndex = usize;

enum StateEnded {
    Checkmate {winner: PlayerIndex},
    Resignation {winner: PlayerIndex},
    WinOnTime {winner: PlayerIndex},
    Forfeit {winner: PlayerIndex},
    Draw(StateEndedDraw),
}

enum StateEndedDraw {
    DrawByAgreement,
    Stalemate,
    ThreefoldRepetition,
    FiftyMoveRule,
    DeadPosition,
    DrawOnTime,
}

type TileRow = Vec<Tile>;

pub struct Board {
    grid: Vec<TileRow>,
}

struct Tile {
    position: Position,
    piece: Option<Piece>,
}

struct Position {
    // (0, 0) is bottom left for white
    x: usize,
    y: usize,
}

struct File {
    n: usize,
}

struct Rank {
    n: usize,
}

struct Piece {
    kind: PieceKind,
    player: PlayerIndex,
    color: Color,
}

enum PieceKind {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Copy, Clone)]
pub enum PrintStyle {
    Ascii,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            board: Board::new(),
            state: State::Active,
            players: [
                Player::new(Color::White),
                Player::new(Color::Black),
            ],
        };

        game.setup_initial_board_pieces();

        game
    }
    fn player_white(&self) -> PlayerIndex { 0 }
    fn player_black(&self) -> PlayerIndex { 1 }
    fn setup_initial_board_pieces(&mut self) {
        // TODO: assert call only once?
        let pawn = Piece::new(PieceKind::Pawn, self.player_white(), self);
        let tile = &mut self.board.grid[0][0];
        tile.piece = Option::Some(pawn);
    }
}

impl Player {
    fn new(color: Color) -> Player {
        Player {
            color,
            captured: vec![],
        }
    }
}

impl PieceKind {
    fn letter (&self) -> &str {
        self.ascii_meta().0
    }
    fn ascii (&self, color: Color) -> &str {
        match color {
            Color::White => self.ascii_meta().2,
            Color::Black => self.ascii_meta().1,
        }
    }
    fn value (&self) -> u32 {
        self.ascii_meta().3
    }
    fn ascii_meta (&self) -> (&str, &str, &str, u32) {
        match self {
            PieceKind::King     => ("K", "♔", "♚", 9),
            PieceKind::Queen    => ("Q", "♕", "♛", 9),
            PieceKind::Rook     => ("R", "♖", "♜", 5),
            PieceKind::Knight   => ("N", "♘", "♞", 3),
            PieceKind::Bishop   => ("B", "♗", "♝", 3),
            PieceKind::Pawn     => ("P", "♙", "♟", 1),
        }
    }
}

impl Board {
    fn new() -> Board {
        Board {
            grid: (0..8).map(|row| {
                (0..8).map(|col| Tile {
                    position: Position {x: col, y: row},
                    piece: Option::None,
                }).collect()
            }).collect(),
        }
    }
    pub fn print(&self, style: PrintStyle) -> String {
        assert!(self.grid.len()>0);

        let border = true;
        let number = true;

        let inner = self.grid.iter().rev().enumerate().map(|(y, row)| {
            let inner = row.iter().map(|tile| {
                tile.print(style)
            }).collect::<Vec<String>>().join("");

            let nr = y+1;
            match (number, border) {
                (true, true) => format!(" {} │{}│", nr, inner),
                (false, true) => format!("│{}│", inner),
                (true, false) => format!(" {} {}", nr, inner),
                (false, false) => format!("{}", inner),
            }
        }).collect::<Vec<String>>().join("\n");

        let cols = self.grid[0].len();
        // TODO: resolve "closure is different" reuse/DRY issue
        let join_cols1 = |f| (0..cols).map(f).collect::<Vec<_>>().join("");
        let join_cols2 = |f| (0..cols).map(f).collect::<Vec<_>>().join("");
        let y_border = join_cols1(|_x| "───");
        let nr_row = join_cols2(|x| format!(" {} ", File::new(x).print(style)));

        if !border {
            match number {
                true => format!("{}\n{}", inner, nr_row),
                false => format!("{}", inner),
            }
        } else {
            match number {
                true => format!("   ╭{}╮\n{}\n   ╰{}╯\n    {} ", y_border, inner, y_border, nr_row),
                false => format!("   ╭{}╮\n{}\n   ╰{}╯", y_border, inner, y_border),
            }
        }
    }

    /*
      ╭────────────────────────╮
    8 │ .     .     .     .    │
    7 │    .     .     .     . │
    6 │ .     .     .     .    │
    5 │    .     .     .     . │
    4 │ .     .     .     .    │
    3 │    .     .     .     . │
    2 │ .     .     .     .    │
    1 │    .     .     .     . │
      ╰────────────────────────╯
        A  B  C  D  E  F  G  H 
    */
}

impl Tile {
    fn color(&self) -> Color {
        let checker_pattern_color_same_as_bottom_left_for_white = self.position.x % 2 == self.position.y % 2;
        match checker_pattern_color_same_as_bottom_left_for_white {
            true => Color::Black,
            false => Color::White,
        }
    }
    fn print(&self, style: PrintStyle) -> String {
        match style {
            PrintStyle::Ascii => {
                match &self.piece {
                    Option::Some(piece) => format!(" {} ", piece.kind.ascii(piece.color)),
                    Option::None => match self.color() {
                        Color::White => " . ".to_owned(),
                        Color::Black => "   ".to_owned(),
                    },
                }
            }
        }
    }
}

impl Piece {
    fn new(kind: PieceKind, player: PlayerIndex, game: &Game) -> Piece {
        Piece {
            kind,
            player,
            color: game.players[player].color,
        }
    }
}

impl File {
    fn new(n: usize) -> File {
        File {
            n,
        }
    }
    fn print(&self, style: PrintStyle) -> String {
        assert!(self.n<15);
        match style {
            PrintStyle::Ascii => {
                String::from_utf8_lossy(&[(self.n as u8)+b'A']).to_owned().to_string()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
