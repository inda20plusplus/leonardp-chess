// TODO: moves
// TODO: use mod + pub
// TODO: use test (unit)
// TODO: use test (full GM replay using PGN)

struct Game {
    board: Board,
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


struct Board {
    grid: Vec<Vec<Tile>>,
}

struct Tile {
    piece: Piece,
}

struct Piece {
    kind: PieceKind,
    player: PlayerIndex,
}

enum PieceKind {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

impl PieceKind {
    fn letter (&self) -> &str {
        match self {
            PieceKind::King     => "K",
            PieceKind::Queen    => "Q",
            PieceKind::Rook     => "R",
            PieceKind::Knight   => "N",
            PieceKind::Bishop   => "B",
            PieceKind::Pawn     => "P",
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
