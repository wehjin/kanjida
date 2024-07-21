use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use chrono::{DateTime, Utc};
use kanji_data::KanjiData;

use crate::game::{AnswerPoint, QuizPoint, YomiPoint};
use crate::game::answer_state::AnswerState;
use crate::game::quiz_state::QuizState;

#[derive(Clone, Default)]
pub struct QuizStates(pub Vec<QuizState>);
impl Debug for QuizStates {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "QuizStates({})", self.0.len())
	}
}

/// Holds game state.
#[derive(Debug, Clone, Default)]
pub struct GameState {
	/// Available quizzes.
	pub all_quizzes: QuizStates,
	/// The selected quiz.
	pub selected_quiz: Option<QuizPoint>,
	/// The selected yomi point. This is used when submitting an answer.
	pub selected_yomi: YomiPoint,
	/// An unused answer point.
	pub unused_answer_point: AnswerPoint,
	/// Submitted answers.
	pub submitted_answers: HashMap<AnswerPoint, AnswerState>,
	/// The age of the game. This advances with each event.
	pub age: usize,
}

impl GameState {
	pub fn quiz_hint(&self, quiz_point: QuizPoint) -> &'static str {
		let quiz = &self.all_quizzes.0[quiz_point];
		quiz.as_hint()
	}
	pub fn as_quiz_states(&self) -> &Vec<QuizState> {
		&self.all_quizzes.0
	}
}

impl GameState {
	/// Initialize the game.
	pub fn init() -> Self {
		let quizzes = (0..KanjiData::len())
			.into_iter()
			.map(|kanji_point| QuizState::init(kanji_point))
			.collect()
			;
		let quiz_states = QuizStates(quizzes);
		GameState { all_quizzes: quiz_states, ..GameState::default() }
	}

	/// Select a quiz.
	pub fn select_quiz(self, quiz_point: QuizPoint) -> Self {
		match quiz_point < self.all_quizzes.0.len() {
			true => {
				let selected_quiz = Some(quiz_point);
				let age = self.age + 1;
				GameState { selected_quiz, age, ..self }
			}
			false => self
		}
	}

	/// Select the yomi to use in the next answer.
	pub fn select_yomi(self, yomi_point: YomiPoint) -> Self {
		let selected_yomi = yomi_point;
		let age = self.age + 1;
		GameState { selected_yomi, age, ..self }
	}

	/// Submit an answer.
	pub fn submit_answer(self) -> (Self, Option<AnswerPoint>) {
		match self.selected_quiz {
			None => (self, None),
			Some(quiz_point) => {
				let yomi_point = self.selected_yomi;
				let answer = AnswerState { quiz_point, yomi_point };
				let answer_point = self.unused_answer_point;
				let mut submitted_answers = self.submitted_answers.clone();
				submitted_answers.insert(answer_point, answer);
				let unused_answer_point = self.unused_answer_point + 1;
				let age = self.age + 1;
				(GameState { unused_answer_point, submitted_answers, age, ..self }, Some(answer_point))
			}
		}
	}

	/// Grade a submitted answer.
	pub fn grade_answer(mut self, answer_point: AnswerPoint, now: DateTime<Utc>) -> Self {
		let answer = self.submitted_answers.get(&answer_point);
		if answer.is_none() {
			return self;
		}
		let &AnswerState { quiz_point, yomi_point } = answer.unwrap();
		self.submitted_answers.remove(&answer_point);
		if quiz_point >= self.all_quizzes.0.len() {
			return self;
		}
		let quiz = self.all_quizzes.0.remove(quiz_point);
		let new_quiz = quiz.attempt_solution(yomi_point, now);
		self.all_quizzes.0.insert(quiz_point, new_quiz);
		self.age = self.age + 1;
		self
	}
}