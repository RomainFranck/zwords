use crate::position::Position;

pub enum ModifierKind {
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord
}

pub struct Modifier {
    pub pos: Position,
    pub kind: ModifierKind
}
