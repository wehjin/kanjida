use std::collections::HashMap;
use kanji_data::KanjiData;
use chrono::{DateTime, Utc};
use crate::game::quiz_state::QuizState;
use crate::game::{AnswerPoint, QuizPoint, YomiPoint};
use crate::game::answer_state::AnswerState;

/// Holds game state.
#[derive(Debug, Clone, Default)]
pub struct GameState {
	/// Available quizzes.
	pub quiz_states: Vec<QuizState>,
	/// The selected quiz.
	pub selected_quiz_point: Option<QuizPoint>,
	/// The selected yomi point. This is used when submitting an answer.
	pub selected_yomi_point: YomiPoint,
	/// An unused answer point.
	pub unused_answer_point: AnswerPoint,
	/// Submitted answers.
	pub submitted_answer_states: HashMap<AnswerPoint, AnswerState>,
	/// The age of the game. This advances with each event.
	pub age: usize,
}

impl GameState {
	pub fn quiz_hint(&self, quiz_point: QuizPoint) -> &'static str {
		let quiz = &self.quiz_states[quiz_point];
		quiz.as_hint()
	}
	pub fn as_quiz_states(&self) -> &Vec<QuizState> {
		&self.quiz_states
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
		GameState { quiz_states: quizzes, ..GameState::default() }
	}

	/// Select a quiz.
	pub fn select_quiz(self, quiz_point: QuizPoint) -> Self {
		match quiz_point < self.quiz_states.len() {
			true => {
				let selected_quiz = Some(quiz_point);
				let age = self.age + 1;
				GameState { selected_quiz_point: selected_quiz, age, ..self }
			}
			false => self
		}
	}

	/// Select the yomi to use in the next answer.
	pub fn select_yomi(self, yomi_point: YomiPoint) -> Self {
		let selected_yomi = yomi_point;
		let age = self.age + 1;
		GameState { selected_yomi_point: selected_yomi, age, ..self }
	}

	/// Submit an answer.
	pub fn submit_answer(self) -> Self {
		match self.selected_quiz_point {
			None => self,
			Some(quiz_point) => {
				let yomi_point = self.selected_yomi_point;
				let answer = AnswerState { quiz_point, yomi_point };
				let answer_point = self.unused_answer_point;
				let mut submitted_answers = self.submitted_answer_states.clone();
				submitted_answers.insert(answer_point, answer);
				let unused_answer_point = self.unused_answer_point + 1;
				let age = self.age + 1;
				GameState { unused_answer_point, submitted_answer_states: submitted_answers, age, ..self }
			}
		}
	}

	/// Grade a submitted answer.
	pub fn grade_answer(mut self, answer_point: AnswerPoint, now: DateTime<Utc>) -> Self {
		let answer = self.submitted_answer_states.get(&answer_point);
		if answer.is_none() {
			return self;
		}
		let &AnswerState { quiz_point, yomi_point } = answer.unwrap();
		self.submitted_answer_states.remove(&answer_point);
		if quiz_point >= self.quiz_states.len() {
			return self;
		}
		let quiz = self.quiz_states.remove(quiz_point);
		let new_quiz = quiz.attempt_solution(yomi_point, now);
		self.quiz_states.insert(quiz_point, new_quiz);
		self.age = self.age + 1;
		self
	}
}