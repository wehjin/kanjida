use std::collections::HashSet;

use chrono::{DateTime, Utc};

use crate::game::quiz::answer::{Answer, AnswerEvent};
use crate::ka::KanjiRecord;

#[derive(Debug, Clone)]
pub struct Quiz {
	question: String,
	answers: Vec<Answer>,
	fails: Vec<DateTime<Utc>>,
}

impl Quiz {
	pub fn glyph(&self) -> &str {
		&self.question
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
	pub fn to_goals(&self) -> HashSet<&String> {
		self.answers.iter().map(|answer| &answer.goal).collect::<HashSet<_>>()
	}
}

fn is_active_fail(fail: &DateTime<Utc>, now: DateTime<Utc>) -> bool {
	(now - fail).num_minutes() <= 3
}

pub enum QuizEvent {
	Solve(String, DateTime<Utc>)
}

impl Quiz {
	pub fn new(record: &KanjiRecord) -> Self {
		let question = record.kanji.to_owned();
		let answers = record.to_onyomi_ja_vec().iter()
			.map(Answer::new)
			.collect::<Vec<_>>()
			;
		let fails = Vec::new();
		Self { question, answers, fails }
	}
	pub fn after_event(&self, event: QuizEvent) -> Self {
		match event {
			QuizEvent::Solve(solution, now) => {
				let mut new = self.clone();

				new.answers = new.answers.into_iter()
					.map(|answer| {
						answer.after_event(AnswerEvent::Solve(solution.to_owned(), now))
					})
					.collect::<Vec<_>>();

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

	pub enum AnswerEvent {
		Solve(String, DateTime<Utc>)
	}
	#[derive(Debug, Clone)]
	pub struct Answer {
		pub age: usize,
		pub goal: String,
		pub recent_solution: Option<DateTime<Utc>>,
	}
	impl Answer {
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
	impl Answer {
		pub fn new(goal: impl AsRef<str>) -> Self {
			Self { age: 0, goal: goal.as_ref().to_string(), recent_solution: None }
		}
		pub fn after_event(&self, event: AnswerEvent) -> Self {
			match event {
				AnswerEvent::Solve(solution, now) => {
					let mut new = self.clone();
					if solution == self.goal {
						new.age += 1;
						new.recent_solution = Some(now);
					}
					new
				}
			}
		}
	}
}

