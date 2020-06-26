pub struct Player {
    pub id: u32,
    pub name: String,
    letters: Vec<char>,
    score: u32
}

impl Player {
    fn addToScore(points: u32) {
	self.score += points;
    }

    fn getScore() -> u32 {
	self.score
    }
}
