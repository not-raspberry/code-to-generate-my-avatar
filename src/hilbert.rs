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

pub struct HilbertCurvePath {
    rules_stack: Vec<(Rule, usize)>,
    max_depth: usize
}

impl HilbertCurvePath {
    pub fn new(order: usize) -> HilbertCurvePath {
        HilbertCurvePath {
            rules_stack: vec![(RULE_A, 0)],
            max_depth: order
        }
    }
}

impl Iterator for HilbertCurvePath {
    type Item = Movement;

    fn next(&mut self) -> Option<Movement> {
        let current_rule: Rule;
        let current_index: usize;
        {
            match self.rules_stack.last_mut() {
                Some(current_rule_and_index) => {
                    let (rule, index) = *current_rule_and_index;
                    current_rule = rule;
                    current_index = index;
                    current_rule_and_index.1 += 1;
                },
                None => {
                    return None
                }
            }
        }
        match current_rule.get(current_index) {
            Some(&symbol) => match symbol {
                Move(movement) => Some(movement),
                Invoke(invocation) => {
                    if self.rules_stack.len() <= self.max_depth {
                        self.rules_stack.push((invocation.to_rule(), 0));
                    }
                    self.next()
                }
            },
            None => {
                self.rules_stack.pop();
                self.next()
            }
        }
    }
}


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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Pos { x: i32, y: i32 }

pub fn move_forward(previous_position: Pos, direction: Direction) -> Pos {
    let Pos { x, y } = previous_position;
    // 0 ---→
    // |   +x
    // |
    // ↓ +y
    match direction {
        Up => Pos { x: x, y: y - 1 },
        Right => Pos { x: x + 1, y: y },
        Down => Pos { x: x, y: y + 1 },
        Left => Pos { x: x - 1, y: y },
    }
}

pub fn hilbert_pixels() {
    let mut position = Pos { x: 0, y: 0 };
    let mut direction = Down;

    for symbol in HilbertCurvePath::new(3) {
        match symbol {
            Movement::Turn(turn_dir) => {
                direction = turn(direction, turn_dir);
            }
            Movement::Forward => {
                position = move_forward(position, direction);
                println!("{:?}", position);
            }
        }
    }
}
