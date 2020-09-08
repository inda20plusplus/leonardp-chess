#![allow(dead_code)]

// TODO: use "trait objects" // https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
//  (eg. instead of PieceKind enum + Piece struct, use Piece trait)
// TODO: look over usage of refs (eg. RefCell) - should some use Rc instead, or other variant?

use std::cell::RefCell;


// game

struct Game {
    players: Vec<RefCell<Player>>,
    pieces: Vec<RefCell<Piece>>,
    state: State,
    turns: Vec<Turn>,
    actions: Vec<RefCell<Action>>,
    board: Board2d,
}

enum Color {
    Black,
    White,
}

struct Player {
    color: Color,
}

enum State {
    Active,
    Ended {
        winner: RefCell<Player>,
    },
}


// piece

enum PieceKind {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

struct Piece {
    kind: PieceKind,
    player: RefCell<Player>,
}


// board

struct Board2d {
    tiles: Vec<RefCell<Tile>>,
    pieces: Vec<RefCell<TilePiece>>,
    edges: Vec<TileEdge>,
}

struct Tile {
    pieces: Vec<RefCell<TilePiece>>,
    color: TileColor,
}

type TileColor = Color;

struct TileEdge {
    origin: RefCell<Tile>,
    target: RefCell<Tile>,
    kind: TileEdgeKind,
}

struct TilePiece {
    piece: RefCell<Piece>,
    tile: RefCell<Tile>,
}


// edge_kind

enum CardinalDirection {
    North,
    East,
    South,
    West,
}
enum InterCardinalDirection {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

enum TileEdgeKind {
    CardinalDirection(CardinalDirection),
    InterCardinalDirection(InterCardinalDirection),
}

type TileEdgeKindPath = Vec<TileEdgeKind>;


// action

enum ActionKind {
    PieceMove {
        piece: RefCell<Piece>,
        path: TileEdgeKindPath,
    },
    Castling {
        rook: RefCell<Piece>,
    },
    Promotion(RefCell<Piece>, PieceKind),
}

struct Action {
    player: RefCell<Player>,
    kind: ActionKind,
}

// type Timestamp = u32; // TODO

struct Turn {
    player: RefCell<Player>,
    actions: Vec<RefCell<Action>>,
    // start_at: Timestamp,
    // end_at: Timestamp,
}


// impl

impl Game {
    fn new() -> Game {
        let players = vec![Player {
            color: Color::White,
        }, Player {
            color: Color::Black,
        }].into_iter().map(RefCell::new).collect();

        let state = State::Active;

        let mut game = Game {
            players,
            pieces: vec![],
            state,
            turns: vec![],
            actions: vec![],
            board: Board2d::new(),
        };

        game.setup_chess_normal();

        game
    }
    fn setup_chess_normal(&mut self) {
        assert_eq!(self.players.len(), 2);
        assert_eq!(self.pieces.len(), 0);

        self.board.setup_grid(8, 8);

        // create pieces + assign to player + tiles
        unimplemented!();
    }
    fn current_player(&self) -> &Player {
        // look at self.turns.last.player or player.0 if empty
        unimplemented!()
    }
}

impl Board2d {
    fn new() -> Board2d {
        Board2d {
            tiles: vec![],
            pieces: vec![],
            edges: vec![],
        }
    }
    fn setup_grid(&mut self, rows: usize, cols: usize) {
        assert_eq!(self.tiles.len(), 0);
        if rows==0 || cols==0 {return;}

        // create tiles + edges + color
        unimplemented!();
    }
}


// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let game = Game::new();
        assert_eq!(game.players.len(), 2);
    }
}
