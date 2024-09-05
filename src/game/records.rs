use crate::game::states::game_state::QuizStates;
use crate::game::states::quiz_state::QuizState;
use crate::game::states::solution_state::SolutionState;
use aframers::browser::log;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use web_sys::{window, Storage};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LessonRecord {
	pub kanji_point: usize,
	pub part_records: Vec<LessonPartRecord>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct LessonPartRecord {
	pub yomi_prefix: char,
	pub completion_date: Option<DateTime<Utc>>,
}

pub fn write(quiz_states: &QuizStates) {
	let records = quiz_states.to_lesson_records();
	let json = serde_json::to_string(&records).unwrap();
	local_storage().set_item("quiz_state1", &json).unwrap();
	log("Wrote changed history");
}

fn local_storage() -> Storage {
	let local_storage = window().unwrap().local_storage().unwrap().unwrap();
	local_storage
}

pub fn read() -> QuizStates {
	let json = local_storage().get_item("quiz_state1").ok().flatten();
	let records = json.map(|json| serde_json::from_str::<Vec<LessonRecord>>(&json).ok()).flatten();
	let quiz_states = records.map(QuizStates::from_lesson_records);
	match quiz_states {
		None => {
			log("Initializing new history");
			QuizStates::new()
		}
		Some(quiz_states) => {
			log("Reading existing history");
			quiz_states
		}
	}
}

impl QuizState {
	fn from_lesson_record(record: &LessonRecord) -> Self {
		Self {
			kanji_point: record.kanji_point,
			solutions: record.part_records.iter().map(|part_record| {
				let LessonPartRecord { yomi_prefix, completion_date } = part_record;
				let solution_state = SolutionState {
					recent_success: completion_date.clone()
				};
				(*yomi_prefix, solution_state)
			}).collect(),
		}
	}
	fn to_lesson_record(&self) -> LessonRecord {
		LessonRecord {
			kanji_point: self.kanji_point,
			part_records: self.solutions.iter()
				.map(|(yomi_prefix, solution_state)| {
					LessonPartRecord {
						yomi_prefix: *yomi_prefix,
						completion_date: solution_state.recent_success.clone(),
					}
				})
				.collect(),
		}
	}
}

impl QuizStates {
	fn from_lesson_records(records: impl AsRef<[LessonRecord]>) -> Self {
		let records = records.as_ref();
		let quiz_states = records.iter().map(QuizState::from_lesson_record).collect();
		Self(quiz_states)
	}
	fn to_lesson_records(&self) -> Vec<LessonRecord> {
		self.0.iter().map(QuizState::to_lesson_record).collect()
	}
}
