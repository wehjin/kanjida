use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::af_sys::scenes::AScene;
use aframers::browser::{document, log};
use aframers::components::core::ComponentValue;
use aframers::components::Position;
use chrono::Utc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;
use web_sys::js_sys::Function;

use AnimationEvent::AnimationComplete;
use GameEvent::{SelectQuiz, SelectYomi, SubmitAnswer};

use crate::aframe_ex::af_sys::{AEntityEx, ASceneEx};
use crate::aframe_ex::components::animation_component::{Animation, AnimationEvent, Easing};
use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::js::log_value;
use crate::aframe_ex::scenes::Scene;
use crate::aframe_ex::schema::settings::ComponentAttribute;
use crate::ecs::components::game_component::GameEvent::GradeAnswer;
use crate::ecs::entities::create_sprite_entity;
use crate::ecs::entities::hint_entity::get_hint_cursor;
use crate::GAME;
use crate::game::{AnswerPoint, QuizPoint, YomiPoint};
use crate::queries::quiz_form_from_point;
use crate::views::{element_id_from_answer_point, element_id_from_quiz_point, element_selector_from_answer_point};

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
			SelectQuiz => "game-select-quiz",
			SelectYomi => "game-select-yomi",
			SubmitAnswer => "game-submit-answer",
			GradeAnswer => "game-grade-answer",
		}
	}
}

pub fn register_game_component() {
	let events = Events::new()
		.set_handler(SelectQuiz, select_quiz)
		.set_handler(SelectYomi, select_yomi)
		.set_handler(SubmitAnswer, submit_answer)
		.set_handler(GradeAnswer, grade_answer)
		;
	ComponentDefinition::new()
		.set_events(events)
		.register("game");
}
fn grade_answer(comp: AComponent, event: CustomEvent) {
	log_value(&event);
	// Update state
	let game = GAME.take();
	let answer_point = event.detail().as_f64().unwrap() as AnswerPoint;
	let game_state = game.grade_answer(answer_point, Utc::now());
	log(&format!("GRADE_ANSWER: {:?}", &game_state));
	GAME.set(game_state);
	// Update entities.
	{
		let selector = element_selector_from_answer_point(answer_point);
		if let Ok(Some(sprite)) = comp.a_entity().a_scene().query_selector(&selector) {
			sprite.remove();
		}
	}
	render_hint_cursor();
}

fn submit_answer(comp: AComponent, _event: CustomEvent) {
	let game = GAME.take();
	let (game_state, answer_point) = game.submit_answer();
	log(&format!("SUBMIT_ANSWER: {:?}", &game_state));
	GAME.set(game_state);

	if let Some(answer_point) = answer_point {
		render_answer_sprite(answer_point, comp.a_entity().a_scene());
	}
}

fn select_yomi(_comp: AComponent, event: CustomEvent) {
	let yomi_point = event.detail().as_f64().map(|detail| detail as usize).unwrap_or(0);
	let game_state = GAME.take().select_yomi(yomi_point);
	log(&format!("SELECT_YOMI: {:?}", &game_state));
	GAME.set(game_state);

	let selected_yomi = GAME.with_borrow(|game_state| game_state.selected_yomi);
	render_yomigun(selected_yomi);
}

fn select_quiz(_comp: AComponent, event: CustomEvent) {
	log_value(&event);
	let quiz_point = event.detail().as_f64().map(|it| it as usize).unwrap_or(0);
	let game_state = GAME.take().select_quiz(quiz_point);
	log(&format!("SELECT_QUIZ: {:?}", &game_state));
	GAME.set(game_state);

	let selected_quiz = GAME.with_borrow(|game_state| game_state.selected_quiz);
	render_hexgrid(selected_quiz);
	render_hint_cursor();
}

fn render_hint_cursor() {
	let selected_quiz = GAME.with_borrow(|game| game.selected_quiz);
	if let Some(quiz_point) = selected_quiz {
		let quiz_form = quiz_form_from_point(quiz_point);
		let hint = get_hint_cursor();
		hint.set_attribute(
			quiz_form.as_attribute_name().as_ref(),
			quiz_form.as_attribute_str().as_ref(),
		).unwrap();
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
		{
			let a_scene = scene.a_scene().clone();
			sprite.a_entity()
				.add_event_listener_with_callback(
					AnimationComplete.as_ref(),
					&Closure::once_into_js(move |event: CustomEvent| {
						log_value(&event);
						a_scene.emit_event_with_details(GradeAnswer.as_ref(), &answer_point.into())
					}).unchecked_into::<Function>(),
				).unwrap();
		}
		scene.add_entity(sprite).unwrap();
	}
}

pub struct Game;
impl ComponentValue for Game {
	fn component_name(&self) -> &str { "game" }
	fn component_value(&self) -> impl AsRef<str> { "true" }
}


