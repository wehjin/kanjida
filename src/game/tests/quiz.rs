use chrono::Utc;

use crate::game::quiz::{Quiz, QuizEvent};

#[test]
fn check() {
	let now = Utc::now();
	let quiz = Quiz::new(4);
	assert_eq!(quiz.answers_len(), 2);
	assert_eq!(quiz.unsolved_answers_len(now), 2);
	assert_eq!(quiz.fails_len(now), 0);
	let quiz = quiz.after_event(QuizEvent::TrySolution("いち".into(), now));
	assert_eq!(quiz.answers_len(), 2);
	assert_eq!(quiz.unsolved_answers_len(now), 2);
	assert_eq!(quiz.fails_len(now), 1);
	let quiz = quiz.after_event(QuizEvent::TrySolution("シ".into(), now));
	assert_eq!(quiz.answers_len(), 2);
	assert_eq!(quiz.unsolved_answers_len(now), 1);
	assert_eq!(quiz.fails_len(now), 1);
}
