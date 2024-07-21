use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::af_sys::scenes::AScene;
use aframers::browser::{document, log};
use aframers::components::core::ComponentValue;
use aframers::components::Position;
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;

use crate::aframe_ex::af_sys::{AEntityEx, ASceneEx};
use crate::aframe_ex::components::animation_component::{Animation, Easing};
use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::js::log_value;
use crate::aframe_ex::scenes::Scene;
use crate::ecs::entities::create_sprite_entity;
use crate::GAME;
use crate::game::{AnswerPoint, QuizPoint, YomiPoint};
use crate::views::{element_id_from_answer_point, element_id_from_quiz_point};

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
	log_value(&event);
	let quiz_point = event.detail().as_f64().map(|it| it as usize).unwrap_or(0);
	let game_state = GAME.take().select_quiz(quiz_point);
	log(&format!("SELECT_QUIZ: {:?}", &game_state));
	GAME.set(game_state);

	let selected_quiz = GAME.with_borrow(|game_state| game_state.selected_quiz);
	render_hexgrid(selected_quiz);
}
fn handle_select_yomi(_comp: AComponent, event: CustomEvent) {
	let yomi_point = event.detail().as_f64().map(|detail| detail as usize).unwrap_or(0);
	let game_state = GAME.take().select_yomi(yomi_point);
	log(&format!("SELECT_YOMI: {:?}", &game_state));
	GAME.set(game_state);

	let selected_yomi = GAME.with_borrow(|game_state| game_state.selected_yomi);
	render_yomigun(selected_yomi);
}

fn handle_submit_answer(comp: AComponent, _event: CustomEvent) {
	let game = GAME.take();
	let (game_state, answer_point) = game.submit_answer();
	log(&format!("SUBMIT_ANSWER: {:?}", &game_state));
	GAME.set(game_state);

	if let Some(answer_point) = answer_point {
		render_answer_sprite(answer_point, comp.a_entity().a_scene());
	}
}
fn render_hexgrid(selected_quiz: Option<QuizPoint>) {
	if let Some(quiz_point) = selected_quiz {
		let cell_selector = format!("#{}", element_id_from_quiz_point(quiz_point));
		let cell_element = document().query_selector(&cell_selector).unwrap().unwrap();
		cell_element.unchecked_ref::<AEntity>().add_state("selected");
	}
}
fn render_yomigun(selected_yomi: YomiPoint) {
	let yomigun = document().query_selector("#yomigun").unwrap().unwrap();
	yomigun.unchecked_ref::<AEntity>()
		.update_component_property("yomigun", "yomiCode", &selected_yomi.into());
}

fn render_answer_sprite(answer_point: AnswerPoint, a_scene: AScene) {
	let scene = Scene::from(a_scene);
	let yomigun_target_vector = scene.a_scene().unchecked_ref::<ASceneEx>().yomigun_target_position();
	let yomigun = document().query_selector("#yomigun").unwrap().unwrap().unchecked_into::<AEntityEx>();
	if let Some(target) = yomigun_target_vector {
		let end = Position(target.x(), target.y(), target.z());
		let start = {
			let relative_position = Position(0., 0., -1.);
			yomigun.local_position_to_world(relative_position)
		};
		let animation = Animation::new()
			.set_property("position")
			.set_from(start)
			.set_to(end)
			.set_dur_millis(250)
			.set_easing(Easing::EaseOutQuad)
			;
		let id = element_id_from_answer_point(answer_point);
		let sprite = create_sprite_entity(start)
			.set_id(id).unwrap()
			.set_component(animation).unwrap();
		scene.add_entity(sprite).unwrap();
	}
}

pub struct Game;
impl ComponentValue for Game {
	fn component_name(&self) -> &str { "game" }
	fn component_value(&self) -> impl AsRef<str> { "true" }
}


