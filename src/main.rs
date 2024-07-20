use std::cell::RefCell;

use game::game_state::GameState;

pub mod aframe_ex;
pub mod three_sys;
mod ecs;
mod game;
mod run;
mod views;

thread_local! {
	pub static GAME: RefCell<GameState> = RefCell::new(GameState::init());
}

fn main() {
	console_error_panic_hook::set_once();
	aframers::init(run::run).expect("Aframe init");
}
