use crate::game::game_state::GameState;
use crate::queries::query_details_at_kanji_point;

pub struct GameMaterial {
	pub details: String,
}

impl GameMaterial {
	pub fn derive(game: &GameState) -> Self {
		let details = derive_details(&game);
		Self { details }
	}
}

fn derive_details(game: &GameState) -> String {
	let details = game.selected_quiz
		.map(|quiz_point| {
			let quiz = &game.all_quizzes.0[quiz_point];
			if quiz.is_revealed {
				query_details_at_kanji_point(quiz.kanji_point)
			} else {
				"".to_string()
			}
		})
		.unwrap_or_default()
		;
	details
}
