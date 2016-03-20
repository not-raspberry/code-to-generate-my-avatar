#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Turn {
    Right,
    Left,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Movement {
    Turn(Turn),
    Forward
}

pub type Rule = [Symbol; 11];

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Invocation {
    A,
    B
}

impl Invocation {
    fn to_rule(self) -> Rule {
        match self {
            Invocation::A => RULE_A,
            Invocation::B => RULE_B
        }
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Symbol {
    Move(Movement),
    Invoke(Invocation),
}
use self::Symbol::*;


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

pub const RULE_A: Rule = [
    Move(Movement::Turn(Turn::Left)),
    Invoke(Invocation::B),
    Move(Movement::Forward),
    Move(Movement::Turn(Turn::Right)),
    Invoke(Invocation::A),
    Move(Movement::Forward),
    Invoke(Invocation::A),
    Move(Movement::Turn(Turn::Right)),
    Move(Movement::Forward),
    Invoke(Invocation::B),
    Move(Movement::Turn(Turn::Left)),
];
pub const RULE_B: Rule = [
    Move(Movement::Turn(Turn::Right)),
    Invoke(Invocation::A),
    Move(Movement::Forward),
    Move(Movement::Turn(Turn::Left)),
    Invoke(Invocation::B),
    Move(Movement::Forward),
    Invoke(Invocation::B),
    Move(Movement::Turn(Turn::Left)),
    Move(Movement::Forward),
    Invoke(Invocation::A),
    Move(Movement::Turn(Turn::Right)),
];

pub fn parse_rule(rule: &Rule, depth: u64) {
    if depth == 0 {
        return;
    }
    for symbol in rule {
        match symbol {
            &Invoke(x) => parse_rule(&x.to_rule(), depth - 1),
            _ => println!("{:?}", symbol)
        }
    }
}
