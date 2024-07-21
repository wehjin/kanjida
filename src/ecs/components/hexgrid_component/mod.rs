use aframers::af_sys::components::AComponent;
use aframers::components::core::ComponentValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::events::StateEventKind;
use crate::aframe_ex::schema::properties::Field;
use crate::aframe_ex::schema::single_property::SinglePropertySchema;
use crate::ecs::components::hexgrid_component::other::SelectedEntity;

pub mod handlers;
pub mod other;
pub mod registration;

pub fn register_hexgrid_component() {
	let events = Events::new()
		.set_handler(StateEventKind::StateAdded, handlers::handle_state_added)
		.set_handler(StateEventKind::StateRemoved, handlers::handle_state_removed)
		;
	let schema = SinglePropertySchema::from(Field::string(Hexgrid::Enabled.as_ref()));
	ComponentDefinition::new()
		.set_events(events)
		.set_schema(schema)
		.set_init_remove_with_extra_state(registration::init, registration::remove)
		.register(HEXGRID_COMPONENT_NAME);
}

pub const HEXGRID_COMPONENT_NAME: &str = "hexgrid";

pub enum Hexgrid { Enabled }

impl AsRef<str> for Hexgrid {
	fn as_ref(&self) -> &str {
		match self { Hexgrid::Enabled => "enabled" }
	}
}


impl ComponentValue for Hexgrid {
	fn component_name(&self) -> &str { HEXGRID_COMPONENT_NAME }
	fn component_value(&self) -> impl AsRef<str> { "enabled" }
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AComponent)]
	pub type HexgridAComponent;
	#[wasm_bindgen(method, getter, js_name = extra_state)]
	pub fn selected_entity(this: &HexgridAComponent) -> SelectedEntity;
	#[wasm_bindgen(method, setter, js_name = extra_state)]
	pub fn set_selected_entity(this: &HexgridAComponent, value: SelectedEntity);
}
