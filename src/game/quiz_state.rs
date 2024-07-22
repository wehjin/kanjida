use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use kanji_data::KanjiData;

use crate::game::{KanjiPoint, YomiPoint};
use crate::game::solution_state::SolutionState;
use crate::views::yomi_data::{first_char_in_str, YomiChar};

/// Holds the state of a quiz.
#[derive(Debug, Clone)]
pub struct QuizState {
	/// Points to a kanji character.
	pub kanji_point: KanjiPoint,
	/// Solutions to the quiz.
	pub solutions: HashMap<char, SolutionState>,
}

impl QuizState {
	pub fn as_hint(&self) -> &'static str {
		KanjiData(self.kanji_point).as_meaning()
	}
	pub fn as_question(&self) -> &'static str {
		KanjiData(self.kanji_point).as_glyph()
	}
}

impl QuizState {
	pub fn init(kanji_point: KanjiPoint) -> Self {
		let data = KanjiData(kanji_point);
		let onyomi = data.as_onyomi();
		let unique_first_chars = onyomi.iter()
			.map(|&yomi| first_char_in_str(yomi))
			.collect::<HashSet<_>>()
			;
		let mut ordered_first_chars = unique_first_chars.into_iter().collect::<Vec<_>>();
		ordered_first_chars.sort();
		let solutions = ordered_first_chars
			.into_iter()
			.map(|first_char| (first_char, SolutionState::init()))
			.collect::<HashMap<_, _>>()
			;
		Self { kanji_point, solutions }
	}
	pub fn attempt_solution(mut self, yomi_point: YomiPoint, now: DateTime<Utc>) -> Self {
		let search_ch = YomiChar(yomi_point).to_char();
		if let Some(solution) = self.solutions.remove(&search_ch) {
			let solution = solution.succeed(now);
			self.solutions.insert(search_ch, solution);
		}
		self
	}
}