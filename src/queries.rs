use crate::ecs::components::quiz_form_component::quiz_form::QuizForm;
use crate::GAME;
use crate::game::game_state::GameState;
use crate::game::QuizPoint;

pub fn quiz_form_from_point(quiz_point: QuizPoint) -> QuizForm {
	let quiz_form = GAME.with_borrow(|game| {
		quiz_form_from_point_game(quiz_point, game)
	});
	quiz_form
}

fn quiz_form_from_point_game(quiz_point: QuizPoint, game: &GameState) -> QuizForm {
	let quiz = &game.all_quizzes[quiz_point];
	let (unsolved, solved) = quiz.score();
	QuizForm { unsolved, solved }
}