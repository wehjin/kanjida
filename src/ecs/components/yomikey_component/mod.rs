use aframers::af_sys::entities::AEntity;
use aframers::browser::{document, log};
use aframers::components::core::ComponentValue;
use wasm_bindgen::{JsCast, JsValue};

use crate::aframe_ex::af_sys::AEntityEx;
use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::cursor_component::CursorEvent::{Click, MouseEnter, MouseLeave};
use crate::aframe_ex::schema::{Field, SinglePropertySchema};
use crate::ecs::components::yomikey_component::bindgen::YomikeyAComponent;
use crate::ecs::components::yomikey_component::yk_settings::YkeySetting;
use crate::ecs::components::yomikey_component::yk_state::YkeyState;

const YOMIKEY_COMPONENT: &'static str = "yomikey";

#[derive(Debug, Copy, Clone)]
pub struct Yomikey(pub usize);
impl ComponentValue for Yomikey {
	fn component_name(&self) -> &str { YOMIKEY_COMPONENT }
	fn component_value(&self) -> impl AsRef<str> { format!("{}", self.0) }
}

mod bindgen;

pub fn register_yomikey_component() {
	let events = Events::new()
		.set_handler(MouseEnter, handle_enter)
		.set_handler(MouseLeave, handle_leave)
		.set_handler(Click, handle_click)
		;
	let schema = SinglePropertySchema::from(Field::usize(0));
	ComponentDefinition::new()
		.set_events(events)
		.set_schema(schema)
		.set_init_remove_ref(handle_init, handle_remove)
		.register(YOMIKEY_COMPONENT)
	;
}

#[derive(Debug, Copy, Clone)]
enum YomikeyEffect {
	SelectYomi(usize)
}

fn handle_click(comp: YomikeyAComponent, _event: JsValue) {
	let state = comp.take_rust_state().click();
	log(&format!("CLICK: {:?}", &state));
	let effects = vec![YomikeyEffect::SelectYomi(state.glyph())];
	do_effects(effects);
	comp.set_rust_state(state);
}

fn do_effects(effects: Vec<YomikeyEffect>) {
	for effect in effects {
		log(&format!("EFFECT: {:?}", effect));
		match effect {
			YomikeyEffect::SelectYomi(yomi_code) => {
				let yomigun = document().query_selector("#yomigun").unwrap().unwrap();
				yomigun.unchecked_ref::<AEntity>()
					.update_component_property("yomigun", "yomiCode", &yomi_code.into());
			}
		}
	}
}

fn handle_enter(comp: YomikeyAComponent, _event: JsValue) {
	let state = comp.take_rust_state().enter();
	log(&format!("ENTER: {:?}", &state));
	update_entity(&state, comp.a_entity().unchecked_ref::<AEntityEx>());
	comp.set_rust_state(state);
}

fn handle_leave(comp: YomikeyAComponent, _event: JsValue) {
	let state = comp.take_rust_state().leave();
	update_entity(&state, comp.a_entity().unchecked_ref::<AEntityEx>());
	comp.set_rust_state(state);
}

fn handle_init(comp: &YomikeyAComponent) {
	let state = YkeyState::init([YkeySetting::Glyph(comp.yomi_code())]);
	update_entity(&state, comp.a_entity().unchecked_ref::<AEntityEx>());
	comp.set_rust_state(state);
}

fn handle_remove(comp: &YomikeyAComponent) {
	comp.take_rust_state();
}

fn update_entity(state: &YkeyState, entity: &AEntityEx) {
	let color = match state.is_focused() {
		true => "Chartreuse",
		false => "ForestGreen"
	};
	entity.set_attribute("color", color).unwrap();
}

pub mod yk_settings;
pub mod yk_state;
