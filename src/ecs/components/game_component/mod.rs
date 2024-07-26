use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::af_sys::scenes::AScene;
use aframers::browser::{document, log};
use aframers::components::core::ComponentAttribute;
use aframers::components::Position;
use aframers::entities::Entity;
use chrono::Utc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;
use web_sys::js_sys::Function;

use AnimationEvent::AnimationComplete;
use GameEvent::{GradeAnswer, SelectQuiz, SelectYomi, SubmitAnswer, ToggleSolution};

use crate::aframe_ex::af_sys::{AEntityEx, ASceneEx};
use crate::aframe_ex::components::animation_component::{Animation, AnimationEvent, Easing};
use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::oculus_touch_controls_component::OculusTouchControlsEvent::AButtonDown;
use crate::aframe_ex::js::log_value;
use crate::aframe_ex::scenes::Scene;
use crate::aframe_ex::Value;
use crate::ecs::components::hexcell_component::attribute::Hexcell;
use crate::ecs::components::quiz_form_component::quiz_form::QuizForm;
use crate::ecs::entities::{create_sprite_entity, hint_entity};
use crate::ecs::entities::hint_entity::get_hint_cursor;
use crate::GAME;
use crate::game::{AnswerPoint, QuizPoint, YomiPoint};
use crate::game::game_state::GameState;
use crate::game::quiz_state::QuizState;
use crate::queries::quiz_form_from_point;
use crate::views::{element_id_from_answer_point, element_id_from_quiz_point, element_selector_from_answer_point};

pub mod game;

pub fn register_game_component() {
	let events = Events::new()
		.set_handler(SelectQuiz, select_quiz)
		.set_handler(SelectYomi, select_yomi)
		.set_handler(SubmitAnswer, submit_answer)
		.set_handler(GradeAnswer, grade_answer)
		.set_handler(ToggleSolution, toggle_solution)
		.set_handler(AButtonDown, on_a_button_down)
		;
	ComponentDefinition::new()
		.set_events(events)
		.register("game");
}

fn on_a_button_down(comp: AComponent, event: CustomEvent) {
	log_value(&event);
	comp.a_entity().a_scene().emit_event(ToggleSolution.as_ref());
}

fn toggle_solution(_comp: AComponent, event: CustomEvent) {
	let quiz_point = update_game("TOGGLE_SOLUTION", event, |mut game, _event| {
		match game.selected_quiz {
			Some(quiz_point) => {
				game.all_quizzes = game.all_quizzes.swap(quiz_point, QuizState::toggle_revealed);
				(game, Some(quiz_point))
			}
			None => (game, None),
		}
	});
	render_hint_cursor_and_quiz_status();
	if let Some(quiz_point) = quiz_point {
		let hint = GAME.with_borrow(|game| game.quiz_hint(quiz_point));
		hint_entity::get()
			.set_component(Value(hint.to_uppercase())).unwrap()
		;
	}
}

fn grade_answer(_comp: AComponent, event: CustomEvent) {
	let answer_point = update_game("GRADE_ANSWER", event, |state, event| {
		let answer_point = event.detail().as_f64().unwrap() as AnswerPoint;
		(state.grade_answer(answer_point, Utc::now()), answer_point)
	});
	// Update entities.
	erase_answer_sprite(answer_point);
	render_hint_cursor_and_quiz_status();
}

fn submit_answer(comp: AComponent, event: CustomEvent) {
	let answer_point = update_game("SUBMIT_ANSWER", event, |state, _event| {
		state.submit_answer()
	});
	if let Some(answer_point) = answer_point {
		render_answer_sprite(answer_point, comp.a_entity().a_scene());
	}
}

fn select_yomi(_comp: AComponent, event: CustomEvent) {
	update_game("SELECT_YOMI", event, |state, event| {
		let yomi_point = event.detail().as_f64().map(|detail| detail as usize).unwrap_or(0);
		let game_state = state.select_yomi(yomi_point);
		(game_state, ())
	});
	let selected_yomi = GAME.with_borrow(|game_state| game_state.selected_yomi);
	render_yomigun(selected_yomi);
}

fn select_quiz(_comp: AComponent, event: CustomEvent) {
	update_game("SELECT_QUIZ", event, |state, event| {
		let quiz_point = event.detail().as_f64().map(|it| it as usize).unwrap_or(0);
		let game_state = state.select_quiz(quiz_point);
		(game_state, ())
	});
	let selected_quiz = GAME.with_borrow(|game_state| game_state.selected_quiz);
	render_hexgrid(selected_quiz);
	render_hint_cursor_and_quiz_status();
}

fn update_game<T>(name: impl AsRef<str>, event: CustomEvent, step: impl Fn(GameState, CustomEvent) -> (GameState, T)) -> T {
	log_value(&event);
	let state = GAME.take();
	let (new_state, effects) = step(state, event);
	log(&format!("{}: {:?}", name.as_ref(), &new_state));
	GAME.set(new_state);
	effects
}

fn erase_answer_sprite(answer_point: AnswerPoint) {
	let selector = element_selector_from_answer_point(answer_point);
	if let Ok(Some(sprite)) = document().query_selector(&selector) {
		sprite.remove();
	}
}

fn render_hint_cursor_and_quiz_status() {
	let selected_quiz = GAME.with_borrow(|game| game.selected_quiz);
	if let Some(quiz_point) = selected_quiz {
		let quiz_form = quiz_form_from_point(quiz_point);
		render_hint_cursor(quiz_form);
		render_quiz_status(quiz_point, quiz_form);
	}
}

fn render_quiz_status(quiz_point: QuizPoint, quiz_form: QuizForm) {
	if quiz_form.unsolved == 0 {
		if let Some(quiz) = document().get_element_by_id(&element_id_from_quiz_point(quiz_point)) {
			Entity::from(quiz.unchecked_into::<AEntity>())
				.set_component(Hexcell::new().set_state(1)).unwrap()
			;
		}
	}
}

fn render_hint_cursor(quiz_form: QuizForm) {
	let hint = get_hint_cursor();
	hint.set_attribute(
		quiz_form.as_attribute_name().as_ref(),
		quiz_form.as_attribute_str().as_ref(),
	).unwrap();
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
	/// Reveals or hides the solution for the selected quiz.
	ToggleSolution,
}

impl AsRef<str> for GameEvent {
	fn as_ref(&self) -> &str {
		match self {
			SelectQuiz => "game-select-quiz",
			SelectYomi => "game-select-yomi",
			SubmitAnswer => "game-submit-answer",
			GradeAnswer => "game-grade-answer",
			ToggleSolution => "game-toggle-solution",
		}
	}
}