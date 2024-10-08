use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::browser::{document, log};
use aframers::components::Position;
use chrono::Utc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;
use web_sys::js_sys::Function;

use AnimationEvent::AnimationComplete;
use GameEvent::{GradeAnswer, SelectQuiz, SelectYomi, SubmitAnswer, ToggleSolution};
use SelectedQuizState::{Selected, Unselected};

use crate::aframe_ex::components::animation_component::{Animation, AnimationEvent, Easing};
use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::oculus_touch_controls_component::OculusTouchControlsEvent::AButtonDown;
use crate::aframe_ex::events::core::AEvent;
use crate::aframe_ex::js::log_value;
use crate::aframe_ex::scene_entity_bindgen::ASceneEx;
use crate::aframe_ex::scenes::A_SCENE;
use crate::aframe_ex::scenes::core::scene_apply_effects;
use crate::aframe_ex::schema::properties::Vec3SchemaProperty;
use crate::ecs::entities::create_sprite_entity;
use crate::ecs::systems::platform_system::platform_system_animate_to_view_selected_hexcell;
use crate::GAME;
use crate::game::AnswerPoint;
use crate::game::game_material::GameMaterial;
use crate::game::game_view::derive_scene_effects;
use crate::game::states::game_state::GameState;
use crate::game::states::selected_quiz_state::SelectedQuizState;
use crate::views::{answer_point_element_selector, element_id_from_answer_point};

pub mod game;

pub fn register_game_component() {
	let events = Events::new()
		.set_handler(SelectQuiz, on_select_quiz)
		.set_handler(SelectYomi, on_select_yomi)
		.set_handler(SubmitAnswer, on_submit_answer)
		.set_handler(GradeAnswer, on_grade_answer)
		.set_handler(ToggleSolution, on_toggle_solution)
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

fn on_toggle_solution(_comp: AComponent, event: CustomEvent) {
	update_game("TOGGLE_SOLUTION", event, |mut game, _event| {
		let selected_quiz = match game.selected_quiz {
			Selected { quiz_point, revealed } => Selected { quiz_point, revealed: !revealed },
			Unselected => game.selected_quiz,
		};
		game.selected_quiz = selected_quiz;
		(game, ())
	});
	render_scene();
}

fn on_grade_answer(_comp: AComponent, event: CustomEvent) {
	let answer_point = update_game("GRADE_ANSWER", event, |state, event| {
		let answer_point = event.detail().as_f64().unwrap() as AnswerPoint;
		(state.grade_answer(answer_point, Utc::now()), answer_point)
	});
	// Update entities.
	erase_answer_sprite(answer_point);
	render_scene();
}

fn on_submit_answer(_comp: AComponent, event: CustomEvent) {
	let position = Vec3SchemaProperty::try_position(&event.detail()).expect("start position");
	let answer_point = update_game(
		"SUBMIT_ANSWER",
		event,
		|state, _event| state.submit_answer(),
	);
	if let Some(answer_point) = answer_point {
		render_answer_sprite(answer_point, position);
	}
}

fn on_select_yomi(_comp: AComponent, event: CustomEvent) {
	update_game("SELECT_YOMI", event, |state, event| {
		let yomi_point = event.detail().as_f64().map(|detail| detail as usize).unwrap_or(0);
		let game_state = state.select_yomi(yomi_point);
		(game_state, ())
	});
	render_scene();
}

fn on_select_quiz(_comp: AComponent, event: CustomEvent) {
	update_game("SELECT_QUIZ", event, |mut game, event| {
		let quiz_point = event.detail().as_f64().map(|it| it as usize).unwrap_or(0);
		let selected_quiz = Selected { quiz_point, revealed: false };
		game.selected_quiz = selected_quiz;
		(game, ())
	});
	render_scene();
	platform_system_animate_to_view_selected_hexcell();
}

fn render_scene() {
	let game_material = GAME.with_borrow(GameMaterial::derive);
	let scene_effects = derive_scene_effects(&game_material);
	scene_apply_effects(&ASceneEx::get(), scene_effects);
}

fn update_game<T>(name: impl AsRef<str>, event: CustomEvent, step: impl Fn(GameState, CustomEvent) -> (GameState, T)) -> T {
	log_value(&event);
	let game = GAME.take();
	let age = game.age;
	let (stepped_game, effects) = step(game, event);
	let aged_game = GameState { age: age + 1, ..stepped_game };
	log(&format!("{}: {:?}", name.as_ref(), &aged_game));
	GAME.set(aged_game);
	effects
}

fn erase_answer_sprite(answer_point: AnswerPoint) {
	let selector = answer_point_element_selector(answer_point);
	if let Ok(Some(sprite)) = document().query_selector(&selector) {
		sprite.remove();
		sprite.unchecked_ref::<AEntity>().destroy();
	}
}

fn render_answer_sprite(answer_point: AnswerPoint, start_position: Position) {
	let yomisprite_target_vector = A_SCENE.with(|scene| scene.yomisprite_target_position());
	if let Some(target) = yomisprite_target_vector {
		let end = Position(target.x(), target.y(), target.z());
		let (start, scale) = (start_position, Some(0.2));
		let animation = Animation::new()
			.set_property("position")
			.set_from(start)
			.set_to(end)
			.set_dur_millis(250)
			.set_easing(Easing::EaseOutQuad)
			;
		let id = element_id_from_answer_point(answer_point);
		let sprite = create_sprite_entity(start, scale)
			.set_id(id).unwrap()
			.set_component_attribute(animation).unwrap()
			.into_a_entity()
			;
		sprite.add_event_listener_with_callback(
			AnimationComplete.as_ref(),
			&Closure::once_into_js(move |event: CustomEvent| {
				log_value(&event);
				GradeAnswer.emit_details(&answer_point.into());
			}).unchecked_into::<Function>(),
		).unwrap();
		A_SCENE.with(|scene| scene.append_child(&sprite).unwrap());
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

impl AEvent for GameEvent {
	fn as_event_name(&self) -> &'static str {
		match self {
			SelectQuiz => "game-select-quiz",
			SelectYomi => "game-select-yomi",
			SubmitAnswer => "game-submit-answer",
			GradeAnswer => "game-grade-answer",
			ToggleSolution => "game-toggle-solution",
		}
	}
}

impl AsRef<str> for GameEvent {
	fn as_ref(&self) -> &str { self.as_event_name() }
}
