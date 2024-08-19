use crate::ecs::components::quiz_form_component::quiz_form::QuizForm;
use crate::game::states::game_state::GameState;
use crate::game::states::quiz_state::QuizState;
use crate::game::states::selected_quiz_state::SelectedQuizState;
use crate::game::QuizPoint;
use crate::queries::query_details_at_kanji_point;
use crate::views::element_id_from_quiz_point;
use crate::views::yomi_data::{first_char_in_str, split_string_first_char};
use kanji_data::KanjiData;

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
	game.with_selected_quiz(|quiz, revealed| {
		let data = KanjiData(quiz.kanji_point);
		let meaning = data.as_meaning();
		let progress = derive_progress(quiz, revealed);
		format!("{}\n\n\n\n{}", meaning, &progress).to_uppercase()
	}).unwrap_or_else(|| String::new())
}

fn derive_progress(quiz: &QuizState, revealed: bool) -> String {
	let data = KanjiData(quiz.kanji_point);
	let mut parts = Vec::new();
	for &onyomi in data.as_onyomi() {
		let chars = onyomi.chars();
		if chars.count() == 0 {
			break;
		}
		let c = first_char_in_str(onyomi);
		let solved = revealed || if let Some(solution) = quiz.solutions.get(&c) {
			solution.is_solved()
		} else {
			false
		};
		let part = match solved {
			false => {
				let (_, rest) = split_string_first_char(onyomi);
				format!("〓{}", &rest)
			}
			true => onyomi.to_string(),
		};
		parts.push(part);
	}
	parts.join(", ")
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
