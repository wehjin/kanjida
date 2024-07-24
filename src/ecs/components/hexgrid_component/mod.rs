use hexgrid::Hexgrid;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::events::StateEventKind;
use crate::aframe_ex::schema::fields::Field;
use crate::aframe_ex::schema::single_property::SinglePropertySchema;
use crate::ecs::components::hexgrid_component::other::SelectedEntity;

pub mod bindgen;
pub mod handlers;
pub mod hexgrid;
pub mod other;
pub mod registration;

pub const HEXGRID_COMPONENT_NAME: &str = "hexgrid";

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
