use super::Tile;

pub fn tile_to_string(tile: Tile) -> String {
    let file: &str = match tile.file {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => "?",
    };
    let rank = tile.rank + 1;
    format!("{}{}", file, rank)
}

pub fn move_to_string(from: Tile, to: Tile) -> String {
    format!("{} {}", tile_to_string(from), tile_to_string(to))
}
