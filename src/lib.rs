pub mod error;

pub mod board;
pub mod position;
pub mod word;
pub mod modifier;
pub mod player;

pub struct Game {
    board: board::Board,
    players: Vec<player::Player>,
    turn: Option<std::iter::Cycle<Player>>
}

impl Game {
    fn new() -> Game {
	let board = board::Board::new();
	let players = Vec::new();
	let turn = None;

	Game { board, players, turn }
    }
    
    fn addPlayer(&mut self, name: &str) -> Result<(), Error> {
	let id = (0u32..).find(|&id| !self.players.iter().any(|player| player.id == id));
	self.players.push(Player { id, name.to_string() });
    }

    fn start(&mut self) {
	self
    }
    
    fn playWord(&mut self)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
