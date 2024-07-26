use aframers::af_sys::scenes::AScene;
use aframers::browser::log;
use aframers::components::core::ComponentAttribute;
use wasm_bindgen::JsCast;
use web_sys::Event;

use crate::aframe_ex::af_sys::AEntityEx;
use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::cursor_component::CursorEvent::{Click, MouseEnter, MouseLeave};
use crate::aframe_ex::schema::fields::Field;
use crate::aframe_ex::schema::single_property::SinglePropertySchema;
use crate::ecs::components::game_component::GameEvent;
use crate::ecs::components::yomikey_component::bindgen::YomikeyAComponent;
use crate::ecs::components::yomikey_component::yk_settings::YkeySetting;
use crate::ecs::components::yomikey_component::yk_state::YkeyState;

const YOMIKEY_COMPONENT: &'static str = "yomikey";

#[derive(Debug, Copy, Clone)]
pub struct Yomikey(pub usize);
impl ComponentAttribute for Yomikey {
	fn as_attribute_name(&self) -> impl AsRef<str> { YOMIKEY_COMPONENT }
	fn as_attribute_str(&self) -> impl AsRef<str> { format!("{}", self.0) }
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

fn handle_click(comp: YomikeyAComponent, _event: Event) {
	let state = comp.take_rust_state().click();
	log(&format!("CLICK: {:?}", &state));
	let effects = vec![YomikeyEffect::SelectYomi(state.glyph())];
	do_effects(effects, comp.a_entity().a_scene());
	comp.set_rust_state(state);
}

fn do_effects(effects: Vec<YomikeyEffect>, a_scene: AScene) {
	for effect in effects {
		log(&format!("EFFECT: {:?}", effect));
		match effect {
			YomikeyEffect::SelectYomi(yomi_point) => {
				a_scene.emit_event_with_details(GameEvent::SelectYomi.as_ref(), &yomi_point.into());
			}
		}
	}
}

fn handle_enter(comp: YomikeyAComponent, _event: Event) {
	let state = comp.take_rust_state().enter();
	log(&format!("ENTER: {:?}", &state));
	update_entity(&state, comp.a_entity().unchecked_ref::<AEntityEx>());
	comp.set_rust_state(state);
}

fn handle_leave(comp: YomikeyAComponent, _event: Event) {
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
