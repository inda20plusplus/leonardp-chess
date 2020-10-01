use crate::piece::*;
use crate::position::*;
// use crate::lib::matcher::*;
use std::str::FromStr;

pub struct PGNCommand {
    pub piece: Option<PieceKind>,
    pub position: Position,
}

impl PGNCommand {
    pub fn from_str(source: &str) -> Option<PGNCommand> {
        // TODO: DRY
        match source.len() {
            3 => {
                let piece = PieceKind::from_str(&source[0..1]);
                let position = Position::from_str(&source[1..3]);

                if let Ok(position) = position {
                    Option::Some(PGNCommand { piece, position })
                } else {
                    Option::None
                }
            }
            2 => {
                let position = Position::from_str(&source[1..3]);

                if let Ok(position) = position {
                    Option::Some(PGNCommand {
                        piece: Option::None,
                        position,
                    })
                } else {
                    Option::None
                }
            }
            _ => Option::None,
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
