use std::cell::RefCell;

use wasm_bindgen::JsValue;

use game::states::game_state::GameState;

use crate::run::register_components;

pub mod aframe_ex;
pub mod js_sys_ex;
pub mod three_sys;
mod ecs;
mod game;
mod run;
mod queries;
mod views;

thread_local! {
	pub static GAME: RefCell<GameState> = RefCell::new(GameState::init());
}

fn main() -> Result<(), JsValue> {
	console_error_panic_hook::set_once();
	register_components();
	Ok(())
}
