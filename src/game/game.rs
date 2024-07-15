use crate::game::quiz::Quiz;
use crate::ka::parse_kanji;

#[derive(Debug)]
pub struct Game {
	quizzes: Vec<Quiz>,
}
impl Game {
	pub fn new() -> Self {
		let quizzes = parse_kanji().iter().map(Quiz::new).collect::<Vec<_>>();
		Self { quizzes }
	}
	pub fn as_quizzes(&self) -> &[Quiz] { self.quizzes.as_slice() }
	pub fn as_quiz(&self, pos: usize) -> &Quiz {
		&self.as_quizzes()[pos]
	}
}
