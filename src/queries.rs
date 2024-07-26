use kanji_data::KanjiData;
use kanji_data::examples::KanjiExample;
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
	let (unsolved, solved, revealed) = quiz.unsolved_solved_revealed();
	QuizForm { unsolved, solved, revealed }
}

pub fn query_details_at_kanji_point(kanji_point: usize) -> String {
	let kd = KanjiData(kanji_point);
	let mut sections = Vec::new();
	let examples = kd.as_examples();
	for example in examples {
		let KanjiExample { sound, meaning, .. } = example;
		let details = format!("{}: {}\n", sound, meaning);
		sections.push(details);
	}
	let details = sections.join("");
	details
}