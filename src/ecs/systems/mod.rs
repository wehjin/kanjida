use wasm_bindgen::prelude::wasm_bindgen;

use crate::aframe_ex::scenes::A_SCENE;
use crate::ecs::components::game_component::GameEvent::SelectQuiz;
use crate::game::QuizPoint;

pub mod game_system;
pub mod hexgrid_system;
pub mod keystaff_system;
pub mod platform_system;


#[wasm_bindgen]
pub fn select_quiz(quiz_point: QuizPoint) {
	A_SCENE.with(|scene| scene.emit_event_with_details(SelectQuiz.as_ref(), &quiz_point.into()))
}