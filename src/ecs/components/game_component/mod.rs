use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::browser::{document, log};
use aframers::components::core::ComponentValue;
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::GAME;

/// Enumerates game events.
#[derive(Debug, Clone)]
pub enum GameEvent {
	/// Selects a quiz to be the target of an answer.
	SelectQuiz,
	/// Selects a yomi to use in the next answer.
	SelectYomi,
	/// Submits an answer using the selected yomi and quiz.
	SubmitAnswer,
	/// Verify an answer against the solutions in its quiz.
	GradeAnswer,
}
impl AsRef<str> for GameEvent {
	fn as_ref(&self) -> &str {
		match self {
			GameEvent::SelectQuiz => "game-select-quiz",
			GameEvent::SelectYomi => "game-select-yomi",
			GameEvent::SubmitAnswer => "game-submit-answer",
			GameEvent::GradeAnswer => "game-grade-answer",
		}
	}
}

pub fn register_game_component() {
	let events = Events::new()
		.set_handler(GameEvent::SelectQuiz, handle_select_quiz)
		.set_handler(GameEvent::SelectYomi, handle_select_yomi)
		.set_handler(GameEvent::SubmitAnswer, handle_submit_answer)
		;
	ComponentDefinition::new()
		.set_events(events)
		.register("game");
}
fn handle_select_quiz(_comp: AComponent, event: CustomEvent) {
	let quiz_point = event.detail().as_f64().map(|it| it as usize).unwrap_or(0);
	let game_state = GAME.take().select_quiz(quiz_point);
	log(&format!("SELECT_QUIZ: {:?}", &game_state));
	GAME.set(game_state);
}
fn handle_select_yomi(_comp: AComponent, event: CustomEvent) {
	let yomi_point = event.detail().as_f64().map(|it| it as usize).unwrap_or(0);
	let game_state = GAME.take().select_yomi(yomi_point);
	log(&format!("SELECT_YOMI: {:?}", &game_state));
	GAME.set(game_state);

	let selected_yomi = GAME.with_borrow(|game_state| game_state.selected_yomi);
	let yomigun = document().query_selector("#yomigun").unwrap().unwrap();
	yomigun.unchecked_ref::<AEntity>()
		.update_component_property("yomigun", "yomiCode", &selected_yomi.into());
}

fn handle_submit_answer(_comp: AComponent, _event: CustomEvent) {
	let game = GAME.take();
	let game_state = game.submit_answer();
	log(&format!("SUBMIT_ANSWER: {:?}", &game_state));
	GAME.set(game_state);
}

pub struct Game;
impl ComponentValue for Game {
	fn component_name(&self) -> &str { "game" }
	fn component_value(&self) -> impl AsRef<str> { "true" }
}


