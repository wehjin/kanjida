pub mod answer_state;
pub mod game_state;
pub mod quiz_state;
pub mod solution_state;

pub mod selected_quiz_state {
	use crate::game::QuizPoint;

	#[derive(Debug, Copy, Clone)]
	pub enum SelectedQuizState {
		Selected { quiz_point: QuizPoint, revealed: bool },
		Unselected,
	}
	impl Default for SelectedQuizState {
		fn default() -> Self {
			SelectedQuizState::Unselected
		}
	}
	impl From<QuizPoint> for SelectedQuizState {
		fn from(value: QuizPoint) -> Self {
			Self::Selected { quiz_point: value, revealed: false }
		}
	}

	impl SelectedQuizState {}
}
