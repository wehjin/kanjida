use aframers::af_sys::components::AComponent;
use aframers::browser::log;
use aframers::components::core::ComponentValue;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::aframe_ex::af_sys::AEntityEx;
use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::cursor_component::CursorEvent::{MouseEnter, MouseLeave};
use crate::ecs::components::yomikey_component::yk_settings::YkeySetting;
use crate::ecs::components::yomikey_component::yk_state::YkeyState;

const YOMIKEY_COMPONENT: &'static str = "yomikey";

pub struct Yomikey;
impl ComponentValue for Yomikey {
	fn component_name(&self) -> &str { YOMIKEY_COMPONENT }
	fn component_value(&self) -> impl AsRef<str> { "true" }
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AComponent)]
	pub type YomikeyAComponent;
	#[wasm_bindgen(method, getter, js_name = rustState)]
	pub fn take_rust_state(this: &YomikeyAComponent) -> YkeyState;
	#[wasm_bindgen(method, setter, js_name = rustState)]
	pub fn set_rust_state(this: &YomikeyAComponent, value: YkeyState);
}

pub fn register_ykey_component() {
	let events = Events::new()
		.set_handler(MouseEnter, handle_enter)
		.set_handler(MouseLeave, handle_leave)
		;
	ComponentDefinition::new()
		.set_events(events)
		.set_init_remove_with_extra_state(
			handle_init,
			handle_remove,
		)
		.register(YOMIKEY_COMPONENT)
	;
}


fn handle_enter(comp: YomikeyAComponent, _event: JsValue) {
	let state = comp.take_rust_state().enter();
	log(&format!("is_focused: {}", state.is_focused()));
	render(&state, comp.a_entity().unchecked_ref::<AEntityEx>());
	comp.set_rust_state(state);
}

fn handle_leave(comp: YomikeyAComponent, _event: JsValue) {
	let state = comp.take_rust_state().leave();
	log(&format!("is_focused: {}", state.is_focused()));
	render(&state, comp.a_entity().unchecked_ref::<AEntityEx>());
	comp.set_rust_state(state);
}

fn render(state: &YkeyState, entity: &AEntityEx) {
	let color = match state.is_focused() {
		true => "Chartreuse",
		false => "ForestGreen"
	};
	entity.set_attribute("color", color).unwrap();
}

fn handle_init(comp: AComponent) {
	let comp = comp.unchecked_ref::<YomikeyAComponent>();
	let state = YkeyState::init([]);
	render(&state, comp.a_entity().unchecked_ref::<AEntityEx>());
	comp.set_rust_state(state);
}
fn handle_remove(comp: AComponent) {
	let comp = comp.unchecked_ref::<YomikeyAComponent>();
	comp.take_rust_state();
}

pub mod yk_settings;
pub mod yk_state;
