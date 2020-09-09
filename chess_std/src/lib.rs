#![allow(dead_code)]

// TODO: moves
// TODO: use mod + pub
// TODO: use test (unit)
// TODO: use test (full GM replay using PGN)
// TODO: replace all unwrap with correct error handling (send to end user)

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

#[derive(PartialEq, Debug)]
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

struct PGNCommand {
    piece: Option<PieceKind>,
    position: Position,
}

impl PGNCommand {
    fn from_str(source: &str) -> Option<PGNCommand> {
        // TODO: DRY
        match source.len() {
            3 => {
                let piece = PieceKind::from_str(&source[0..1]);
                let position = Position::from_str(&source[1..3]);
                
                if let Option::Some(position) = position {
                    Option::Some(PGNCommand {
                        piece,
                        position,
                    })
                } else {
                    Option::None
                }
            },
            2 => {
                let position = Position::from_str(&source[1..3]);
                
                if let Option::Some(position) = position {
                    Option::Some(PGNCommand {
                        piece: Option::None,
                        position,
                    })
                } else {
                    Option::None
                }
            },
            _ => Option::None,
        }
    }
}

impl Position {
    fn from_str(source: &str) -> Option<Position> {
        let mut chars = vec![];

        source.to_uppercase().chars().for_each(|a| chars.push(a));
        if chars.len()!=2 {return Option::None}
        
        let (x, y) = (chars[0] as u8, chars[1] as u8);
        if x < b'A' || x > b'Z' {return Option::None}
        if y < b'0' || y > b'9' {return Option::None}

        let x = x-b'A';
        let y = y-b'1';

        Option::Some(Position { x: x as usize, y: y as usize })
    }
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
        self.add_pieces_from_str("Ra8 Nb8 Bc8 Kd8 Qe8 Bf8 Ng8 Rh8", self.player_black());
        self.add_pieces_from_str("Pa7 Pb7 Pc7 Pd7 Pe7 Pf7 Pg7 Ph7", self.player_black());

        self.add_pieces_from_str("Pa2 Pb2 Pc2 Pd2 Pe2 Pf2 Pg2 Ph2", self.player_white());
        self.add_pieces_from_str("Ra1 Nb1 Bc1 Qd1 Ke1 Bf1 Ng1 Rh1", self.player_white());
    }
    fn add_piece(&mut self, player: PlayerIndex, position: Position, kind: PieceKind) {
        let piece = Piece::new(kind, player, self);
        let tile = &mut self.board.grid[position.y][position.x];
        assert!(tile.piece.is_none());
        tile.piece = Option::Some(piece);
    }
    fn add_pieces_from_str(&mut self, source: &str, player: PlayerIndex) {
        source.split_ascii_whitespace()
            .map(PGNCommand::from_str)
            .map(|x| x.unwrap())
            .for_each(|c| self.add_piece(player, c.position, c.piece.unwrap()));
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
            PieceKind::Pawn     => ("P", "♙", "♟︎", 1),
        }
    }
    fn from_letter (letter: &str) -> Option<PieceKind> {
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
    fn from_str(source: &str) -> Option<PieceKind> {
        Self::from_letter(source)
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
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn pgn_command_simple_parsing() {
        let c = PGNCommand::from_str("Nb5");
        assert!(c.is_some());
        let c = c.unwrap();
        assert_eq!(c.piece.unwrap(), PieceKind::Knight);
        assert_eq!(c.position.x, 1);
        assert_eq!(c.position.y, 4);
    }
}
