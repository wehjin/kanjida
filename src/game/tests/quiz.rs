use chrono::Utc;

use crate::game::quiz::{Quiz, QuizEvent};
use crate::ka::parse_kanji;

#[test]
fn check() {
	let now = Utc::now();
	let kanji = parse_kanji();
	let quiz = Quiz::new(&kanji[4]);
	assert_eq!(quiz.answers_len(), 2);
	assert_eq!(quiz.unsolved_answers_len(now), 2);
	assert_eq!(quiz.fails_len(now), 0);
	let quiz = quiz.after_event(QuizEvent::Solve("いち".into(), now));
	assert_eq!(quiz.answers_len(), 2);
	assert_eq!(quiz.unsolved_answers_len(now), 2);
	assert_eq!(quiz.fails_len(now), 1);
	let quiz = quiz.after_event(QuizEvent::Solve("シ".into(), now));
	assert_eq!(quiz.answers_len(), 2);
	assert_eq!(quiz.unsolved_answers_len(now), 1);
	assert_eq!(quiz.fails_len(now), 1);
}
