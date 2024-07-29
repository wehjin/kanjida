use kanji_data::KanjiData;

use crate::ecs::components::quiz_form_component::quiz_form::QuizForm;
use crate::game::QuizPoint;
use crate::game::states::game_state::GameState;
use crate::game::states::selected_quiz_state::SelectedQuizState;
use crate::queries::query_details_at_kanji_point;
use crate::views::element_id_from_quiz_point;
use crate::views::yomi_data::split_string_first_char;

pub struct GameMaterial {
	pub details: String,
	pub hint: String,
	pub quiz_add_selected: Option<EntityStateAddition>,
	pub quiz_form: Option<QuizForm>,
	pub selected_quiz_point: Option<QuizPoint>,
}

impl GameMaterial {
	pub fn derive(game: &GameState) -> Self {
		Self {
			details: derive_details(game),
			hint: derive_hint(game),
			quiz_add_selected: derive_quiz_add_selected(game),
			quiz_form: derive_quiz_form(game),
			selected_quiz_point: derive_selected_quiz_point(game),
		}
	}
}

fn derive_selected_quiz_point(game: &GameState) -> Option<QuizPoint> {
	match game.selected_quiz {
		SelectedQuizState::Selected { quiz_point, .. } => Some(quiz_point),
		SelectedQuizState::Unselected => None
	}
}

fn derive_quiz_form(game: &GameState) -> Option<QuizForm> {
	match game.selected_quiz {
		SelectedQuizState::Selected { quiz_point, revealed } => Some(
			{
				let quiz = &game.all_quizzes[quiz_point];
				let (unsolved, solved) = quiz.unsolved_solved();
				let revealed = revealed as usize;
				QuizForm { unsolved, solved, revealed }
			}
		),
		SelectedQuizState::Unselected => None,
	}
}

fn derive_quiz_add_selected(game: &GameState) -> Option<EntityStateAddition> {
	match game.selected_quiz {
		SelectedQuizState::Selected { quiz_point, .. } => {
			let selector = format!("#{}", element_id_from_quiz_point(quiz_point));
			let state_name = "selected".to_string();
			Some(EntityStateAddition { entity_selector: selector, state_name })
		}
		SelectedQuizState::Unselected => None,
	}
}

fn derive_hint(game: &GameState) -> String {
	let hint = match game.selected_quiz {
		SelectedQuizState::Selected { quiz_point, revealed } => {
			match revealed {
				true => {
					let quiz = &game.all_quizzes.0[quiz_point];
					let data = KanjiData(quiz.kanji_point);
					let meaning = data.as_meaning();
					let onyomis = derive_onyomi_strings(data);
					format!("{}\n\n\n{}", meaning, &onyomis.join(", "))
				}
				false => String::default(),
			}
		}
		SelectedQuizState::Unselected => String::default(),
	};
	hint.to_uppercase()
}

fn derive_onyomi_strings(data: KanjiData) -> Vec<String> {
	data.as_onyomi().iter()
		.map(|&it| {
			if it.chars().count() > 1 {
				let (first, rest) = split_string_first_char(it);
				format!("{}({})", &first, &rest)
			} else {
				it.to_string()
			}
		})
		.collect::<Vec<_>>()
}

fn derive_details(game: &GameState) -> String {
	match game.selected_quiz {
		SelectedQuizState::Selected { quiz_point, revealed } if revealed => {
			let quiz = &game.all_quizzes[quiz_point];
			query_details_at_kanji_point(quiz.kanji_point)
		}
		_ => String::default(),
	}
}

pub struct EntityStateAddition {
	pub entity_selector: String,
	pub state_name: String,
}
