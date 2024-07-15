use crate::game::game::Game;

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

#[test]
fn check_inventory() {
	let game = Game::new();
	let inventory = game.to_inventory().into_iter().collect::<Vec<_>>();
	assert_eq!(inventory.len(), 280);
}
