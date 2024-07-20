use std::str::FromStr;

use crate::game::QuizPoint;

pub mod settings;
pub mod yomi_data;

pub fn quiz_point_from_element_id(id: impl AsRef<str> + Sized) -> QuizPoint {
	let id = id.as_ref();
	let dash_point = id.rfind("-").unwrap() + 1;
	let quiz_point = usize::from_str(&id[dash_point..]).unwrap() - 1;
	quiz_point
}

pub fn element_id_from_quiz_point(quiz_point: QuizPoint) -> String {
	format!("quiz-{}", quiz_point + 1)
}