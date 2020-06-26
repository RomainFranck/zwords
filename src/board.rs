use crate::error::Error;

use crate::word::Word;
use crate::modifier::Modifier;

pub struct Board {
    size: (i32, i32),
    words: Vec<Word>,
    modifiers: Vec<Modifier>,
}

impl Board {
    fn new(size: (i32, i32), modifiers: Vec<Modifier>) -> Result<Board, Error> {
	let words = Vec::new();

	if modifiers.iter().all(|m: &Modifier| m.pos.x < size.0 && m.pos.y < size.1) {
	    Ok( Board { size, words, modifiers } )
	}
	else {
	    Err(Error::ModifierOutOfBounds)
	}
    }

    fn play(word: mut Word) {
	//word not overlaping

	//word in dictionary

	//take modifiers
    }
}

    
