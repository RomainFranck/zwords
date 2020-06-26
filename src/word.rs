use crate::modifier::Modifier;
use crate::position::Position;

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Word {
    pos: Position,
    word: String,
    direction: Direction,
    modifiers: Vec<Modifier>,
    blanks: Vec<char>
}
