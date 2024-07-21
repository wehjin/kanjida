use std::str::FromStr;

use crate::game::{AnswerPoint, QuizPoint};

pub mod settings;
pub mod yomi_data;

pub fn element_id_from_quiz_point(quiz_point: QuizPoint) -> String {
	format!("{}-{}", "quiz", quiz_point + 1)
}
pub fn quiz_point_from_element_id(id: impl AsRef<str> + Sized) -> QuizPoint {
	let id = id.as_ref();
	let dash_point = id.rfind("-").unwrap() + 1;
	let quiz_point = usize::from_str(&id[dash_point..]).unwrap() - 1;
	quiz_point
}

pub fn element_id_from_answer_point(answer_point: AnswerPoint) -> String {
	format!("{}-{}", "answer", answer_point)
}
pub fn answer_point_from_element_id(id: impl AsRef<str> + Sized) -> AnswerPoint {
	let id = id.as_ref();
	let dash_point = id.rfind("-").unwrap() + 1;
	let answer_point = usize::from_str(&id[dash_point..]).unwrap();
	answer_point
}
