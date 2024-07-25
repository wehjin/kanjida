use std::str::FromStr;

use crate::game::{AnswerPoint, QuizPoint};

pub mod settings;
pub mod yomi_data;

pub fn element_id_from_quiz_point(quiz_point: QuizPoint) -> String {
	format!("{}-{}", "quiz", quiz_point)
}
pub fn quiz_point_from_element_id(id: impl AsRef<str> + Sized) -> QuizPoint {
	let id = id.as_ref();
	let quiz_point = usize::from_str(&id[5..]).unwrap();
	quiz_point
}

pub fn element_id_from_answer_point(answer_point: AnswerPoint) -> String {
	format!("{}-{}", "answer", answer_point)
}
pub fn element_selector_from_answer_point(answer_point: AnswerPoint) -> String {
	format!("#{}", element_id_from_answer_point(answer_point))
}
