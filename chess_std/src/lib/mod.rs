#![allow(dead_code)]

// TODO: moves
// TODO: use mod + pub
// TODO: use test (unit)
// TODO: use test (full GM replay using PGN)
// TODO: replace all unwrap with correct error handling (send to end user)
// TODO: use traits for PieceKind instead of enum (+ register)
// TODO: clean up Some, None, Ok, Err (use directly without ::)
// TODO: split impl into parts and put where relevant sub-types are introduced (eg. Board.print with PrintStyle?)
// TODO: Game::to_pgn().to_string() + parse // https://en.wikipedia.org/wiki/Portable_Game_Notation
//  {turn pair nr}. {white move}{" "+(black move)?}
//  move: Ng3e6#+=Q // Ng3 (origin: Knight or empty for Pawn, file?, rank?)? e6 (target: file?, rank?) # (if checkmate) + (if check) =Q (if promotion to Queen)
// TODO: use Display trait + custom formatter flags
// TODO: terminal interactive mode where pgn interpretation is shown with terminal color (eg. white bold a rank/pieces) + move is tiny animated (show target, show origin, clean)
// TODO: monte-carlo-chain simple computer-player based on grand-masters pgn playback
// TODO: define piece-movements use tile paths according to "Parlett's mevement notation" (https://en.wikipedia.org/wiki/Fairy_chess_piece)
// TODO: implement (https://en.wikipedia.org/wiki/Chess_on_a_really_big_board)
// TODO: add time (inc. modes, eg. "3|2 event, every move you do adds 2 seconds to your clock")

// plan:
// - unit tests for pgn decode (to representation) + encode (back to string) (cases + whole PGN file(s))
// - implement pgn decode + encode
// - pgn to game action + game state to pgn
// - unit test playback + export of pgn + implement
// - use terminal ui to create king-special + king-movement tests pgn + implement
//  - check (copy game, move piece, try echo opponent piece to capture king (+ possibly pawn special), check if successful; then try each own piece to do any valid move, checkmate if unavailable)
//  - castling (king move 2 steps towards rook, rook move to tile between king.origin + king.target; no passing tiles threatened (game.is_tile_threatened(for_player)) + no rook/king moved from game start + empty walk)
// - require pgn '=Q' syntax for promotion + test + implement
// - prevent moves if checkmate, prevent non-un-check-ing moves if check, + change game state if check(mate)
// - integration test to playback gm pgn matches + expect same outcome
// - refactor to use piece trait
// - refactor to use board trait
// - refactor to use action trait + action inverse
// - history redo/undo + use to correct player_moved_from_original_position
// - fisher-chess with arbitrary board size
// - terminal animated pgn playback + upload gif to repo


mod color;
mod pgn;
mod piece;
mod position;
mod view;
mod matcher;

use color::*;
use pgn::*;
use piece::PieceKind;
use position::File;
pub use position::Position;
pub use view::*;

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    state: State,
    players: [Player; 2],
    turns: Vec<Turn>,
}

#[derive(Clone)]
struct Turn {
    player: PlayerIndex,
    actions: Vec<ActionPackage>,
}

#[derive(Debug, Clone)]
enum State {
    Active,
    Ended(StateEnded),
}

#[derive(Clone)]
struct Player {
    color: Color,
    captured: Vec<Piece>,
}

// TODO: use refs instead?
type PlayerIndex = usize;

#[derive(Debug, Clone)]
enum StateEnded {
    Checkmate { winner: PlayerIndex },
    Resignation { winner: PlayerIndex },
    WinOnTime { winner: PlayerIndex },
    Forfeit { winner: PlayerIndex },
    Draw(StateEndedDraw),
}

#[derive(Debug, Clone)]
enum StateEndedDraw {
    DrawByAgreement,
    Stalemate,
    ThreefoldRepetition,
    FiftyMoveRule,
    DeadPosition,
    DrawOnTime,
}

type TileRow = Vec<Tile>;

#[derive(Clone)]
pub struct Board {
    grid: Vec<TileRow>,
}

#[derive(Clone)]
struct Tile {
    position: Position,
    piece: Option<Piece>,
}

#[derive(Clone)]
struct Piece {
    kind: PieceKind,
    player: PlayerIndex,
    color: Color,
}

#[derive(Debug, Clone)]
pub enum Action {
    PieceMove { origin: Position, target: Position },
}

#[derive(Debug, Clone)]
pub struct ActionPackage {
    pub action: Action,
    pub player: PlayerIndex,
}

#[derive(Debug)]
enum ActionValidation {
    Standard,
    EnPassant { capture_tile: Position },
    Promotion,
}

impl Game {
    fn new_black_white(board: Board) -> Game {
        let mut game = Game {
            board,
            state: State::Active,
            players: [Player::new(Color::White), Player::new(Color::Black)],
            turns: vec![],
        };

        game.turns.push(Turn {
            player: game.player_white_index(),
            actions: vec![],
        });

        game
    }
    fn new() -> Game {
        Self::new_black_white(Board::new(8, 8))
    }
    pub fn new_standard_game() -> Game {
        let mut game = Self::new();
        game.setup_standard_board_pieces();
        game
    }
    pub fn new_5x5_empty() -> Game {
        Self::new_black_white(Board::new(5, 5))
    }

    pub fn setup_standard_board_pieces(&mut self) {
        // TODO: assert call only once?
        self.add_pieces_from_str("Ra8 Nb8 Bc8 Kd8 Qe8 Bf8 Ng8 Rh8", self.player_black_index());
        self.add_pieces_from_str("Pa7 Pb7 Pc7 Pd7 Pe7 Pf7 Pg7 Ph7", self.player_black_index());

        self.add_pieces_from_str("Pa2 Pb2 Pc2 Pd2 Pe2 Pf2 Pg2 Ph2", self.player_white_index());
        self.add_pieces_from_str("Ra1 Nb1 Bc1 Qd1 Ke1 Bf1 Ng1 Rh1", self.player_white_index());
    }
    fn add_piece(&mut self, player: PlayerIndex, position: Position, kind: PieceKind) {
        let piece = Piece::new(kind, player, self);
        let tile = &mut self.board.grid[position.y][position.x];
        assert!(tile.piece.is_none());
        tile.piece = Option::Some(piece);
    }
    pub fn add_pieces_from_str(&mut self, source: &str, player: PlayerIndex) {
        source
            .split_ascii_whitespace()
            .map(PGNCommand::from_str)
            .map(|x| x.unwrap())
            .for_each(|c| self.add_piece(player, c.position, c.piece.unwrap()));
    }

    pub fn player_white_index(&self) -> PlayerIndex {
        0
    }
    pub fn player_black_index(&self) -> PlayerIndex {
        1
    }
    pub fn current_player_index(&self) -> PlayerIndex {
        self.turns.last().unwrap().player
    }
    fn current_player(&self) -> &Player {
        &self.players[self.current_player_index()]
    }
    pub fn current_player_title(&self) -> String {
        format!("{:?}", self.current_player().color)
    }

    fn validate_action(&self, action: &ActionPackage) -> Result<ActionValidation, &str> {
        // TODO: ®eturn err message?
        let player = action.player;
        if player != self.current_player_index() {
            return Result::Err("out of turn");
        }
        let action = &action.action;

        match action {
            Action::PieceMove { origin, target } => {
                let origin_tile = self.board.tile_at(*origin).ok_or("invalid origin tile")?;
                let piece = origin_tile.piece.as_ref().ok_or("no piece at origin")?;
                let target_tile = self.board.tile_at(*target).ok_or("invalid target tile")?;

                if piece.player != player {
                    return Result::Err("not players piece at origin");
                }
                if let Option::Some(target_piece) = target_tile.piece.as_ref() {
                    if target_piece.player == player {
                        return Result::Err("players piece at target");
                    }
                }

                let dx = (target.x as i32) - (origin.x as i32);
                let dy = (target.y as i32) - (origin.y as i32);

                piece.kind.delta_move_valid(dx, dy)?;

                // TODO: check special move constraints
                //  eg. limit direction + initial move + diagonal + end rank for pawn
                //  eg. castling for king

                // TODO: move PieceKind specific code to PieceKind
                let action_validation = match piece.kind {
                    PieceKind::Pawn => {
                        let player = &self.players[player];
                        let dy_forward = player.dy_forward();
                        let dy_forward_dir = dy_forward < 0;
                        let move_dy_dir = dy < 0;

                        if move_dy_dir != dy_forward_dir {
                            return Result::Err("pawn cannot move backwards");
                        }

                        if dx == 0 && target_tile.piece.is_some() {
                            return Result::Err("pawn cannot capture forward");
                        }

                        if i32::abs(dy) == 2
                            && !player.is_pawn_home(&self.board, origin_tile.position)
                        {
                            return Result::Err("pawn can only two-step-move starting from home");
                        }

                        if i32::abs(dy) > 2 {
                            return Result::Err("pawn cannot move that far");
                        }

                        let attempted_en_passant = i32::abs(dx) == 1 && target_tile.piece.is_none();
                        if !attempted_en_passant {
                            ActionValidation::Standard
                        } else {
                            let prev_turn = self.turns.get(self.turns.len() - 2);

                            let prev_turn = match prev_turn {
                                Option::Some(prev_turn) => prev_turn,
                                _ => {
                                    return Result::Err(
                                        "en_passant only available after another move",
                                    );
                                }
                            };

                            let just_moved_past = prev_turn
                                .actions
                                .iter()
                                .scan(0, |_, action| match action.action {
                                    Action::PieceMove { origin: _, target } => Some(target),
                                })
                                .find(|action_target_pos| {
                                    let pos = action_target_pos;
                                    let same_file = pos.x == target_tile.position.x;
                                    let rank_before = (pos.y as i32)
                                        == (target_tile.position.y as i32) - dy_forward;
                                    same_file && rank_before
                                });

                            let capture_tile_pos = match just_moved_past {
                                Some(inner) => inner,
                                None => {
                                    return Result::Err(
                                        "en_passant only available just after an enabling move",
                                    );
                                }
                            };

                            ActionValidation::EnPassant {
                                capture_tile: capture_tile_pos,
                            }
                        }
                    }
                    PieceKind::King => {
                        // TODO
                        ActionValidation::Standard
                    }
                    _ => ActionValidation::Standard,
                };

                if !piece.kind.jumps() {
                    let steps = piece.kind.delta_steps(dx, dy);
                    let mut pos = origin_tile.position;
                    for step in steps {
                        // make step
                        pos.x = ((pos.x as i32) + step.0) as usize;
                        pos.y = ((pos.y as i32) + step.1) as usize;
                        // check
                        let is_destination_tile = pos == target_tile.position;
                        if is_destination_tile {
                            break;
                        }

                        let intermediate_tile =
                            self.board.tile_at(pos).ok_or("invalid intermediate tile")?;
                        if intermediate_tile.piece.is_some() {
                            return Result::Err("a piece was in the way");
                        }
                    }
                }

                Ok(action_validation)
            }
        }
    }

    // TODO: make validate_action return wrapper (ValidatedActionPackage)
    //  that may be performed directly?
    pub fn perform_action(&mut self, action: ActionPackage) -> Result<(), String> {
        let action_validaton = match self.validate_action(&action) {
            Ok(inner) => inner,
            Err(e) => return Result::Err(e.to_owned()),
        };

        match action.action {
            Action::PieceMove { origin, target } => {
                let player = &mut self.players[self.current_player_index()];

                let piece = {
                    let origin_tile = self.board.tile_at_mut(origin).unwrap();
                    origin_tile.piece.take().unwrap()
                };
                let captured = {
                    let target_tile = self.board.tile_at_mut(target).unwrap();
                    target_tile.piece.replace(piece)
                };
                if let Option::Some(captured) = captured {
                    player.captured.push(captured);
                }

                match action_validaton {
                    ActionValidation::Standard => (),
                    ActionValidation::EnPassant { capture_tile } => {
                        let capture_tile = self.board.tile_at_mut(capture_tile).unwrap();
                        let captured = capture_tile.piece.take().unwrap();
                        player.captured.push(captured);
                    }
                    ActionValidation::Promotion => {
                        unimplemented!();
                    }
                };

                let current_turn = self.turns.last_mut().unwrap();
                current_turn.actions.push(action);

                let player_next = if self.current_player_index() == 0 {
                    1
                } else {
                    0
                };
                self.turns.push(Turn {
                    player: player_next,
                    actions: vec![],
                });

                Result::Ok(())
            }
        }
    }

    pub fn move_from_str(&self, source: &str) -> Result<ActionPackage, String> {
        let components: Vec<&str> = source.split_ascii_whitespace().collect();
        if components.len() != 2 {
            return Result::Err("expected format like 'a6 b8'".to_owned());
        }
        let ap = ActionPackage {
            player: self.current_player_index(),
            action: Action::PieceMove {
                origin: Position::from_str(&components[0]).ok_or("invalid origin")?,
                target: Position::from_str(&components[1]).ok_or("invalid target")?,
            },
        };
        Result::Ok(ap)
    }

    pub fn status_message(&self) -> String {
        match self.state {
            State::Active => {
                let players = self
                    .players
                    .iter()
                    .map(|p| format!("{:?}({}p)", p.color, p.captured_value()))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{:?}: {}; {:?}'s move",
                    self.state,
                    players,
                    self.current_player().color
                )
            }
            State::Ended(_) => format!("{:?}", self.state),
        }
    }

    fn piece_moved_from_original_position(&self, piece: &Piece, tile: &Tile) -> bool {
        // TODO: simplified, not fully correct implementation
        //  eg. not actually directly keeping track of whether piece has moved, just using lossy heuristics
        // TODO: ability to get piece's current tile from piece

        let player = &self.players[piece.player];
        let is_home = match piece.kind {
            PieceKind::Pawn => player.is_pawn_home(&self.board, tile.position),
            _ => player.home_row(&self.board) == (tile.position.y as i32),
        };

        !is_home
    }
}

impl Player {
    fn new(color: Color) -> Player {
        Player {
            color,
            captured: vec![],
        }
    }
    fn captured_value(&self) -> u32 {
        self.captured.iter().map(|p| p.kind.value()).sum()
    }
    fn dy_forward(&self) -> i32 {
        match self.color {
            Color::White => 1,
            Color::Black => -1,
        }
    }
    fn home_row(&self, board: &Board) -> i32 {
        let last = board.row_count() - 1;
        let row = match self.color {
            Color::White => 0,
            Color::Black => last,
        };
        row as i32
    }
    fn is_pawn_home(&self, board: &Board, pawn_position: Position) -> bool {
        let home_y = self.home_row(board) + self.dy_forward();
        (pawn_position.y as i32) == home_y
    }
}

impl Board {
    fn new(rows: u32, cols: u32) -> Board {
        Board {
            grid: (0..rows)
                .map(|row| {
                    (0..cols)
                        .map(|col| Tile {
                            position: Position {
                                x: col as usize,
                                y: row as usize,
                            },
                            piece: Option::None,
                        })
                        .collect()
                })
                .collect(),
        }
    }
    pub fn print(&self, style: BoardPrintStyle) -> String {
        assert!(!self.grid.is_empty());

        let border = style.border;
        let number = style.number;
        let style = style.style;

        let inner = self
            .grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                let inner = row
                    .iter()
                    .map(|tile| tile.print(style))
                    .collect::<Vec<String>>()
                    .join("");

                let nr = y + 1;
                match (number, border) {
                    (true, true) => format!(" {} │{}│", nr, inner),
                    (false, true) => format!("│{}│", inner),
                    (true, false) => format!(" {} {}", nr, inner),
                    (false, false) => inner,
                }
            })
            .rev()
            .collect::<Vec<String>>()
            .join("\n");

        let cols = self.grid[0].len();
        // TODO: resolve "closure is different" reuse/DRY issue
        let join_cols1 = |f| (0..cols).map(f).collect::<Vec<_>>().join("");
        let join_cols2 = |f| (0..cols).map(f).collect::<Vec<_>>().join("");
        let y_border = join_cols1(|_x| "───");
        let nr_row = join_cols2(|x| format!(" {} ", File::new(x).print(style)));

        if !border {
            match number {
                true => format!("{}\n{}", inner, nr_row),
                false => inner,
            }
        } else {
            match number {
                true => format!(
                    "   ╭{}╮\n{}\n   ╰{}╯\n    {} ",
                    y_border, inner, y_border, nr_row
                ),
                false => format!("╭{}╮\n{}\n╰{}╯", y_border, inner, y_border),
            }
        }
    }

    fn tile_at(&self, position: Position) -> Option<&Tile> {
        self.grid.get(position.y)?.get(position.x)
    }
    fn tile_at_mut(&mut self, position: Position) -> Option<&mut Tile> {
        self.grid.get_mut(position.y)?.get_mut(position.x)
    }

    fn row_count(&self) -> u32 {
        self.grid.len() as u32
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
        let checker_pattern_color_same_as_bottom_left_for_white =
            self.position.x % 2 == self.position.y % 2;
        match checker_pattern_color_same_as_bottom_left_for_white {
            true => Color::Black,
            false => Color::White,
        }
    }
    fn print(&self, style: PrintStyle) -> String {
        match style {
            PrintStyle::Ascii => match &self.piece {
                Option::Some(piece) => format!(" {} ", piece.kind.ascii(piece.color)),
                Option::None => match self.color() {
                    Color::White => " . ".to_owned(),
                    Color::Black => "   ".to_owned(),
                },
            },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_board_setup() {
        let game = Game::new_standard_game();
        let actual = game.board.print(BoardPrintStyle::ascii_bordered());
        assert_eq!(actual, include_str!("../../test_data/board_plain.txt"));
    }

    #[test]
    fn initial_knight_moves() {
        let mut game = Game::new_standard_game();
        game.move_from_str("????").expect_err("invalid format");

        game.perform_action(game.move_from_str("b1 d2").unwrap())
            .expect_err("target tile occupied by players own piece");

        game.perform_action(game.move_from_str("b1 b4").unwrap())
            .expect_err("not a valid knight delta move");

        game.perform_action(game.move_from_str("b1 c3").unwrap())
            .expect("valid move");

        game.perform_action(game.move_from_str("c3 e4").unwrap())
            .expect_err("valid move, but not by current player");

        game.perform_action(game.move_from_str("b8 d9").unwrap())
            .expect_err("outside board");

        game.perform_action(game.move_from_str("b8 a6").unwrap())
            .expect("valid move");
    }

    #[test]
    fn std_move_types() -> Result<(), String> {
        // King: (n, w, e, s, nw, ne, sw, se)*1
        // Queen: (n, w, e, s, nw, ne, sw, se)*inf
        // Rook: (n, w, e, s)*inf
        // Knight: L-shape (any closest tile not on same rank, file or diagonal), jumps_over_other_pieces: true
        // Bishop: (nw, ne, sw, se)*inf

        let mut game = Game::new_standard_game();

        game.perform_action(game.move_from_str("a2 a4")?)?;
        game.perform_action(game.move_from_str("a8 a6")?)
            .expect_err("a piece was in the way");
        // game.perform_action(game.move_from_str("a7 a4")?)
        // 	.expect_err("pawn can only move at max 2 steps initially")
        game.perform_action(game.move_from_str("a7 a6")?)?;
        game.perform_action(game.move_from_str("a1 a3")?)?;
        game.perform_action(game.move_from_str("a6 a5")?)?;
        game.perform_action(game.move_from_str("a3 b4")?)
            .expect_err("rook cannot move diagonally");
        game.perform_action(game.move_from_str("a3 d3")?)?;

        game.perform_action(game.move_from_str("e7 e6")?)?;
        game.perform_action(game.move_from_str("e2 e4")?)?;
        game.perform_action(game.move_from_str("f8 a3")?)?;
        game.perform_action(game.move_from_str("b1 a3")?)?;
        game.perform_action(game.move_from_str("d8 e7")?)?;
        game.perform_action(game.move_from_str("e1 e3")?)
            .expect_err("king cannot move 2 steps");
        game.perform_action(game.move_from_str("d1 h5")?)?;

        assert_eq!(
            game.board.print(BoardPrintStyle::ascii_bordered()),
            include_str!("../../test_data/board_std_moves.txt")
        );
        assert_eq!(
            game.status_message(),
            "Active: White(3p), Black(0p); Black's move"
        );

        Ok(())
    }

    #[test]
    fn pawn_moves() -> Result<(), String> {
        // Pawn: n*1
        // Pawn: n*2 if piece.prev_movements.count=0 / piece at original position
        // Pawn: (nw, ne)*1 if can capture
        // Pawn: en_passant ((nw, ne)*1 if opponent.pawn did n*2 prev_turn and opponent.pawn.file = piece.file)
        // Pawn: promotion (convert (to (Q, R, B, or K) of same color) on move to last rank (ie. required + during same turn))

        let mut game = Game::new_standard_game();

        game.perform_action(game.move_from_str("a2 a5")?)
            .expect_err("pawn can only move at max 2 steps initially");

        game.perform_action(game.move_from_str("a2 a4")?)?; // 2 steps
        game.perform_action(game.move_from_str("b7 b6")?)?; // 1 steps

        game.perform_action(game.move_from_str("a4 a6")?)
            .expect_err("pawn 2 step move only allowed from start pos");
        game.perform_action(game.move_from_str("a4 a3")?)
            .expect_err("no backwards");
        game.perform_action(game.move_from_str("a4 b5")?)
            .expect_err("no diagonal forward without capture (en_passant)");

        game.perform_action(game.move_from_str("a4 a5")?)?;
        game.perform_action(game.move_from_str("b6 a5")?)?; // diagonal capture

        game.perform_action(game.move_from_str("d2 d3")?)?; // white dummy move
        game.perform_action(game.move_from_str("a5 a4")?)?;
        game.perform_action(game.move_from_str("b2 b4")?)?;
        game.perform_action(game.move_from_str("a4 b3")?)?; // en_passant
        game.perform_action(game.move_from_str("b4 b5")?)
            .expect_err("captured using en_passant in previous turn");
        game.perform_action(game.move_from_str("c2 c3")?)?; // en_passant made possible
        game.perform_action(game.move_from_str("h7 h5")?)?; // forfeit en_passant move
        game.perform_action(game.move_from_str("h2 h4")?)?;
        game.perform_action(game.move_from_str("b3 c2")?)
            .expect_err("en_passant only valid immediately after becoming possible");

        game.perform_action(game.move_from_str("h5 h4")?)
            .expect_err("pawn cannot capture forward");

        // TODO: promotion

        Ok(())
    }

    // #[test]
    fn king_moves() -> Result<(), String> {
        // not allowed to move such that player put itself in "check"
        // King: castling (a, h)-side

        let mut game = Game::new();
        game.add_pieces_from_str("Kd8", game.player_black_index());
        game.add_pieces_from_str("Kd8", game.player_white_index());

        // game.perform_action(game.move_from_str("a7 a4")?)
        //     .expect_err("pawn can only move at max 2 steps initially")

        unimplemented!();
        // Ok(())
    }
}
