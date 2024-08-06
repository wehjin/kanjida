use wasm_bindgen::JsCast;
use web_sys::{CustomEvent, Event};

use crate::aframe_ex::events::StateEvent;
use crate::ecs::components::hexcell_component::bindgen::HexcellAComponent;
use crate::ecs::systems::hexgrid_system::{hexgrid_system_clear_selected_cell, hexgrid_system_set_selected_cell};

pub(crate) fn handle_state_added(component: HexcellAComponent, event: Event) {
	let custom = event.unchecked_into::<CustomEvent>();
	if let Some(state_event) = StateEvent::try_added(&custom) {
		match state_event.state() {
			"focused" => component.set_ring_color_from_entity_state(),
			"selected" => {
				component.set_ring_color_from_entity_state();
				hexgrid_system_set_selected_cell(component.a_entity().unchecked_ref());
			}
			_ => ()
		}
	}
}

pub(crate) fn handle_state_removed(component: HexcellAComponent, event: Event) {
	let custom = event.unchecked_into::<CustomEvent>();
	if let Some(state_event) = StateEvent::try_removed(&custom) {
		match state_event.state() {
			"focused" => component.set_ring_color_from_entity_state(),
			"selected" => {
				component.set_ring_color_from_entity_state();
				hexgrid_system_clear_selected_cell(component.a_entity().unchecked_ref())
			}
			_ => ()
		}
	}
}


