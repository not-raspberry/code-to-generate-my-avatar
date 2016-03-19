#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Turn {
    Right,
    Left
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}
pub use self::Direction::*;


/// Given previous direction and turn, computes direction after the turn.
pub fn turn(prev_direction: Direction, turn: Turn) -> Direction {
    match (prev_direction, turn) {
        (Up, Turn::Right) => Right,
        (Up, Turn::Left) => Left,
        (Right, Turn::Right) => Down,
        (Right, Turn::Left) => Up,
        (Down, Turn::Right) => Left,
        (Down, Turn::Left) => Right,
        (Left, Turn::Right) => Up,
        (Left, Turn::Left) => Down,
    }
}
