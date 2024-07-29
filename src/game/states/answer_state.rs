use crate::game::{QuizPoint, YomiPoint};

/// Holds the state of an answer.
#[derive(Debug, Copy, Clone)]
pub struct AnswerState {
	pub quiz_point: QuizPoint,
	pub yomi_point: YomiPoint,
}