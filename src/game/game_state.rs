use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

use chrono::{DateTime, Utc};
use kanji_data::KanjiData;

use crate::game::{AnswerPoint, QuizPoint, YomiPoint};
use crate::game::answer_state::AnswerState;
use crate::game::quiz_state::QuizState;

#[derive(Clone, Default)]
pub struct QuizStates(pub Vec<QuizState>);

impl QuizStates {
	pub fn swap(mut self, index: usize, f: impl Fn(QuizState) -> QuizState) -> Self {
		let state = self.0.remove(index);
		let new_state = f(state);
		self.0.insert(index, new_state);
		self
	}
	pub fn total_unsolved_solved_revealed(&self) -> (usize, usize, usize) {
		let score = self.0.iter().fold(
			(0usize, 0usize, 0usize),
			|(unsolved, solved, revealed), quiz| {
				let (quiz_unsolved, quiz_solved, quiz_revealed) = quiz.unsolved_solved_revealed();
				(unsolved + quiz_unsolved, solved + quiz_solved, revealed + quiz_revealed)
			},
		);
		score
	}
}
impl Debug for QuizStates {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let (unsolved, solved, revealed) = self.total_unsolved_solved_revealed();
		write!(
			f, "QuizStates {{ count: {}, unsolved: {}, solved: {}, revealed: {} }})",
			self.0.len(), unsolved, solved, revealed
		)
	}
}
impl IndexMut<QuizPoint> for QuizStates {
	fn index_mut(&mut self, index: QuizPoint) -> &mut Self::Output {
		&mut self.0[index]
	}
}
impl Index<QuizPoint> for QuizStates {
	type Output = QuizState;
	fn index(&self, index: QuizPoint) -> &Self::Output {
		&self.0[index]
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
	pub fn total_unsolved_solved_revealed(&self) -> (usize, usize, usize) {
		self.all_quizzes.total_unsolved_solved_revealed()
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