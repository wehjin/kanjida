#[cfg(test)]
mod tests;

pub type KanjiPoint = usize;
pub type YomiPoint = usize;
pub type AnswerPoint = usize;
pub type QuizPoint = usize;

/// Enumerates game events.
#[derive(Debug, Clone)]
pub enum GameEvent {
	/// Selects a quiz to be the target of an answer.
	SelectQuiz(QuizPoint),
	/// Selects a yomi to use in the next answer.
	SelectYomi(YomiPoint),
	/// Submits an answer using the selected yomi and quiz.
	SubmitAnswer,
	/// Verify an answer against the solutions in its quiz.
	GradeAnswer(AnswerPoint),
}

pub mod answer_state;
pub mod game_state;
pub mod quiz_state;
pub mod solution_state;

