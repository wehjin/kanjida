use crate::GAME;
use crate::game::game_state::GameState;
use crate::game::QuizPoint;

#[derive(Debug, Clone)]
pub struct QuizForm {
	pub unsolved: usize,
	pub solved: usize,
}
pub fn quiz_form_from_point(quiz_point: QuizPoint) -> QuizForm {
	let quiz_form = GAME.with_borrow(|game| {
		quiz_form_from_point_game(quiz_point, game)
	});
	quiz_form
}

fn quiz_form_from_point_game(quiz_point: QuizPoint, game: &GameState) -> QuizForm {
	let quiz = &game.all_quizzes[quiz_point];
	let (unsolved, solved) = quiz.solutions.iter().fold(
		(0usize, 0usize),
		|(unsolved, solved), (_, solution)| {
			if solution.is_solved() {
				(unsolved, solved + 1)
			} else {
				(unsolved + 1, solved)
			}
		},
	);
	QuizForm { unsolved, solved }
}