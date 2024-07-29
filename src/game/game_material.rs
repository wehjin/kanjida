use kanji_data::KanjiData;

use crate::game::states::game_state::GameState;
use crate::queries::query_details_at_kanji_point;
use crate::views::yomi_data::split_string_first_char;

pub struct GameMaterial {
	pub details: String,
	pub hint: String,
}

impl GameMaterial {
	pub fn derive(game: &GameState) -> Self {
		let details = derive_details(game);
		let hint = derive_hint(game);
		GameMaterial { details, hint }
	}
}

fn derive_hint(game: &GameState) -> String {
	let hint = match game.selected_quiz {
		Some(quiz_point) => {
			let quiz = &game.all_quizzes.0[quiz_point];
			let data = KanjiData(quiz.kanji_point);
			let meaning = data.as_meaning();
			let reveal = match quiz.is_revealed {
				true => {
					let onyomis = data.as_onyomi().iter()
						.map(|&it| {
							if it.chars().count() > 1 {
								let (first, rest) = split_string_first_char(it);
								format!("{}({})", &first, &rest)
							} else {
								it.to_string()
							}
						})
						.collect::<Vec<_>>();
					onyomis.join(", ")
				}
				false => " ".to_string()
			};
			format!("{}\n\n\n{}", meaning, &reveal)
		}
		None => {
			String::default()
		}
	};
	hint.to_uppercase()
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
