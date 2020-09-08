
// game

struct Game<'a> {
    players: Vec<Player>,
    pieces: Vec<Piece<'a>>,
    state: State<'a>,
    turns: Vec<Turn<'a>>,
    actions: Vec<Action<'a>>,
}

enum Color {
    Black,
    White,
}

struct Player {
    color: Color,
}

struct StateEnded<'a> {
    winner: &'a Player,
}

struct StateActive<'a> {
    next_move: &'a Player,
}

enum State<'a> {
    Active(StateActive<'a>),
    Ended(StateEnded<'a>),
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

struct Piece<'a> {
    kind: PieceKind,
    player: &'a Player,
}


// board

struct Board2d<'a> {
    tiles: Vec<Tile<'a>>,
    pieces: Vec<TilePiece<'a>>,
    edges: Vec<TileEdge<'a>>,
}

struct Tile<'a> {
    pieces: Vec<&'a TilePiece<'a>>,
    color: TileColor,
}

type TileColor = Color;

struct TileEdge<'a> {
    origin: Tile<'a>,
    target: Tile<'a>,
    kind: TileEdgeKind,
}

struct TilePiece<'a> {
    piece: &'a Piece<'a>,
    tile: &'a Tile<'a>,
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

enum ActionKind<'a> {
    PieceMove(&'a Piece<'a>, TileEdgeKindPath),
    Castling(&'a Piece<'a>), // with rook
    Promotion(&'a Piece<'a>, PieceKind),
}

struct Action<'a> {
    player: &'a Player,
    kind: ActionKind<'a>,
}

// type Timestamp = u32; // TODO

struct Turn<'a> {
    player: &'a Player,
    actions: Vec<&'a Action<'a>>,
    // start_at: Timestamp,
    // end_at: Timestamp,
}


// tests

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
