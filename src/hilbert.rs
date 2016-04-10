use std::fs::OpenOptions;
use image::{DynamicImage, GenericImage, Rgba, ImageFormat};


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

/// Iterator over turtle graphics commands for the hilbert curve.
pub struct HilbertCurvePath {
    rules_stack: Vec<(Rule, usize)>,
    max_depth: usize
}

impl HilbertCurvePath {
    pub fn new(order: usize) -> HilbertCurvePath {
        HilbertCurvePath {
            rules_stack: vec![(RULE_A, 0)],
            max_depth: order - 1
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
pub struct Position { x: u32, y: u32 }

pub fn move_forward(previous_position: Position, direction: Direction) -> Position {
    let Position { x, y } = previous_position;
    // 0 ---→
    // |   +x
    // |
    // ↓ +y
    match direction {
        Up => Position { x: x, y: y - 1 },
        Right => Position { x: x + 1, y: y },
        Down => Position { x: x, y: y + 1 },
        Left => Position { x: x - 1, y: y },
    }
}

/// Iterator over hilbert curve pixels for bitmaps of 2ⁿ ☓ 2ⁿ pixels.
pub struct HilbertCurvePixels {
    position: Position,
    direction: Direction,
    path: HilbertCurvePath
}

impl HilbertCurvePixels {
    pub fn new(power: u32) -> HilbertCurvePixels {
        HilbertCurvePixels {
            position: Position { x: 0, y: 0 },
            direction: Down,
            path: HilbertCurvePath::new(power as usize)
        }
    }
}

impl Iterator for HilbertCurvePixels {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        let current_position = self.position;

        // Update the next position.
        match self.path.next() {
            Some(movement) => {
                match movement {
                    Movement::Turn(turn_dir) => {
                        self.direction = turn(self.direction, turn_dir);
                        self.next()
                    }
                    Movement::Forward => {
                        self.position = move_forward(self.position, self.direction);
                        Some(current_position)
                    }

                }
            },
            None => {
                // All movements were consumed but we also need to draw the last pixel.
                // Since the wrapped iterator is over at this point, we have to distinguish between
                // the first None returned by the HilbertCurvePath iterator, in which case we have
                // to put the last pixel, and the following Nones, when we have to return None.

                // We set the position in self to a position in which the Hilbert curve can never
                // end (they always start and end in a corner, regardless of their orientation; the
                // corner is always at 2ⁿ-1 ☓ 2ⁿ-1, where n is a natural number, incl. zero).
                let illegal_ending_position = Position { x: 2, y: 2 }; 
                self.position = illegal_ending_position;
                if current_position != illegal_ending_position {
                    Some(current_position)  // Last pixel.
                } else {
                    None  // The last pixel already put - stop the iteration.
                }
            }
        }
    }
}

fn blend(c1: Rgba<u8>, c2: Rgba<u8>, ratio: f32) -> Rgba<u8> {
    assert!(0.0 <= ratio);
    assert!(ratio <= 1.0);
    let c1_strength = ratio;
    let c2_strength = 1.0 - c1_strength;

    let avg = |a, b| (a as f32 * c1_strength  + b as f32 * c2_strength) as u8;

    Rgba(
        [avg(c1.data[0], c2.data[0]),
         avg(c1.data[1], c2.data[1]),
         avg(c1.data[2], c2.data[2]),
         avg(c1.data[3], c2.data[3])]
    )
}

pub fn hilbert_pixels(destination: String) {
    let order: u32 = 8;
    let size: u32  = 2u32.pow(order);
    let pixels_count = size * size;
    let mut image = DynamicImage::new_rgb8(size, size);
    let (initial_color, final_color) = (Rgba([0xe3, 0x0b, 0x5d, 0xff]),
                                        Rgba([0x0, 0x0, 0x0, 0x0]));

    for (index, position) in HilbertCurvePixels::new(order).enumerate() {
        let blend_ratio = index as f32 / pixels_count as f32;
        let color = blend(initial_color, final_color, blend_ratio);
        println!("{:?}", color);
        image.put_pixel(position.x, position.y, color);
    }
    let mut dest = OpenOptions::new().write(true).create(true).truncate(true).open(destination).unwrap();
    image.save(&mut dest, ImageFormat::PNG).unwrap();
}
