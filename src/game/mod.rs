pub use game::*;

mod game;

mod quiz;
#[cfg(test)]
mod tests {
	use crate::game::Game;

	#[test]
	fn init() {
		let game = Game::new();
		assert_eq!(game.as_quizzes().len(), 1235);
	}
	#[test]
	fn quiz() {
		let game = Game::new();
		let quiz = game.as_quiz(4);
		assert_eq!(quiz.glyph(), "示");
	}
}
