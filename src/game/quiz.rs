use chrono::{DateTime, Utc};
use kanji_data::KanjiData;

use crate::game::quiz::answer::{Answer, AnswerEvent, YomiData};

#[derive(Debug, Clone)]
pub struct Quiz {
	data: KanjiData,
	answers: Vec<Answer>,
	fails: Vec<DateTime<Utc>>,
}

impl Quiz {
	pub fn id(&self) -> String { format!("quiz-{}", self.data.0 + 1) }
	pub fn question(&self) -> &'static str { self.data.as_glyph() }
	pub fn hint(&self) -> &'static str {
		&self.data.as_meaning()
	}
	pub fn answers_len(&self) -> usize {
		self.answers.len()
	}
	pub fn solved_answers_len(&self, now: DateTime<Utc>) -> usize {
		self.answers.iter().filter(|&answer| answer.is_solved(now)).count()
	}
	pub fn unsolved_answers_len(&self, now: DateTime<Utc>) -> usize {
		self.answers.len() - self.solved_answers_len(now)
	}
	pub fn fails_len(&self, now: DateTime<Utc>) -> usize {
		self.fails.iter().filter(|&fail| is_active_fail(fail, now)).count()
	}
	pub fn as_goals(&self) -> &'static [&'static str] {
		self.data.as_onyomi()
	}
}

fn is_active_fail(fail: &DateTime<Utc>, now: DateTime<Utc>) -> bool {
	(now - fail).num_minutes() <= 3
}

pub enum QuizEvent {
	TrySolution(String, DateTime<Utc>)
}

impl Quiz {
	pub fn new(pos: usize) -> Self {
		let data = KanjiData(pos);
		let mut answers = Vec::new();
		for pos in 0..data.as_onyomi().len() {
			let answer = Answer::new(YomiData(data, pos));
			answers.push(answer);
		}
		let fails = Vec::new();
		Self { data, answers, fails }
	}
	pub fn after_event(&self, event: QuizEvent) -> Self {
		match event {
			QuizEvent::TrySolution(solution, now) => {
				let mut new = self.clone();
				new.answers = new.answers.into_iter().map(|answer| {
					answer.after_event(AnswerEvent::TrySolution(solution.to_owned(), now))
				}).collect::<Vec<_>>();

				if new.solved_answers_len(now) == self.solved_answers_len(now) {
					new.fails.push(now);
					new.fails = new.fails.into_iter()
						.filter_map(|fail| match is_active_fail(&fail, now) {
							true => Some(fail),
							false => None
						})
						.collect()
					;
				}
				new
			}
		}
	}
}

pub mod answer {
	use chrono::{DateTime, Utc};
	use kanji_data::KanjiData;

	pub enum AnswerEvent {
		TrySolution(String, DateTime<Utc>)
	}

	#[derive(Debug, Copy, Clone)]
	pub struct YomiData(pub KanjiData, pub usize);
	impl YomiData {
		pub fn as_yomi(&self) -> &'static str { self.0.as_onyomi()[self.1] }
	}

	#[derive(Debug, Clone)]
	pub struct Answer {
		pub data: YomiData,
		pub age: usize,
		pub recent_solution: Option<DateTime<Utc>>,
	}
	impl Answer {
		pub fn new(data: YomiData) -> Self {
			Self { data, age: 0, recent_solution: None }
		}

		pub fn after_event(&self, event: AnswerEvent) -> Self {
			match event {
				AnswerEvent::TrySolution(solution, now) => {
					let mut new = self.clone();
					if &solution == self.as_solution() {
						new.age += 1;
						new.recent_solution = Some(now);
					}
					new
				}
			}
		}
	}
	impl Answer {
		pub fn as_solution(&self) -> &'static str { self.data.as_yomi() }
		pub fn is_solved(&self, now: DateTime<Utc>) -> bool {
			if let Some(solution_time) = &self.recent_solution {
				let expired = (now - solution_time).num_days() > 30;
				let solved = !expired;
				solved
			} else {
				false
			}
		}
	}
}

