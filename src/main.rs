use std::cell::RefCell;

use crate::game::game::Game;

pub mod aframe_ex;
pub mod three_sys;
mod ecs;
mod game;
mod load;
mod run;
mod views;

thread_local! {
	pub static GAME: RefCell<Game> = RefCell::new(Game::with_limit(None));
}

fn main() {
	console_error_panic_hook::set_once();
	aframers::init(load::load).expect("Aframe init");
}
