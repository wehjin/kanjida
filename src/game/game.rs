use std::collections::{HashMap, HashSet};

use crate::game::quiz::Quiz;
use crate::ka::parse_kanji;

#[derive(Debug)]
pub struct Game {
	quizzes: Vec<Quiz>,
	by_name: HashMap<String, usize>,
}
impl Game {
	pub fn as_quizzes(&self) -> &[Quiz] { self.quizzes.as_slice() }
	pub fn as_quiz(&self, pos: usize) -> &Quiz {
		&self.as_quizzes()[pos]
	}
	pub fn as_quiz_by_id(&self, name: impl AsRef<str>) -> &Quiz {
		self.as_quiz(self.by_name[name.as_ref()])
	}
	pub fn to_inventory(&self) -> HashSet<&String> {
		let mut set = HashSet::new();
		for quiz in self.as_quizzes() {
			set.extend(quiz.to_goals())
		}
		set
	}
}
impl Game {
	pub fn with_limit(max: Option<usize>) -> Self {
		let mut quizzes = parse_kanji().iter()
			.enumerate()
			.map(Quiz::new)
			.collect::<Vec<_>>()
			;
		if let Some(limit) = max {
			quizzes.truncate(limit);
		}
		let by_name = quizzes.iter()
			.enumerate()
			.map(|(pos, quiz)| {
				(quiz.id().to_string(), pos)
			})
			.collect::<HashMap<_, _>>()
			;
		Self { quizzes, by_name }
	}
	pub fn new() -> Self {
		Self::with_limit(None)
	}
}
