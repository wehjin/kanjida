use std::collections::HashSet;
use std::str::FromStr;

use kanji_data::KanjiData;

use crate::game::quiz::Quiz;

#[derive(Debug)]
pub struct Game {
	quizzes: Vec<Quiz>,
}
impl Game {
	pub fn with_limit(max: Option<usize>) -> Self {
		let mut quizzes = Vec::new();
		for pos in 0..KanjiData::len() {
			let quiz = Quiz::new(pos);
			quizzes.push(quiz);
		}
		if let Some(limit) = max {
			quizzes.truncate(limit);
		}
		Self { quizzes }
	}
	pub fn new() -> Self {
		Self::with_limit(None)
	}
}
impl Game {
	pub fn as_quizzes(&self) -> &[Quiz] { self.quizzes.as_slice() }
	pub fn as_quiz(&self, pos: usize) -> &Quiz {
		&self.as_quizzes()[pos]
	}
	pub fn as_quiz_by_id(&self, id: impl AsRef<str>) -> &Quiz {
		let id = id.as_ref();
		let start = id.rfind("-").unwrap() + 1;
		let pos = usize::from_str(&id[start..]).unwrap() - 1;
		self.as_quiz(pos)
	}
	pub fn to_inventory(&self) -> Vec<&'static str> {
		let mut unique = Vec::new();
		let mut set = HashSet::new();
		let mut count = set.len();
		for quiz in self.as_quizzes() {
			for &goal in quiz.as_goals() {
				set.insert(goal);
				if set.len() > count {
					count += 1;
					unique.push(goal);
				}
			}
		}
		unique
	}
}
