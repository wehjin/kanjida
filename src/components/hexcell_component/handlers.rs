use wasm_bindgen::{JsCast, JsValue};
use web_sys::CustomEvent;

use crate::aframe_ex::events::StateEvent;
use crate::components::hexcell_component::HexcellAComponent;

pub(super) fn handle_state_added(component: HexcellAComponent, event: JsValue) {
	let custom = event.unchecked_into::<CustomEvent>();
	if let Some(state_event) = StateEvent::try_added(&custom) {
		match state_event.state() {
			"focused" | "selected" => component.set_ring_color_from_entity_state(),
			_ => ()
		}
	}
}

pub(super) fn handle_state_removed(component: HexcellAComponent, event: JsValue) {
	let custom = event.unchecked_into::<CustomEvent>();
	if let Some(state_event) = StateEvent::try_removed(&custom) {
		match state_event.state() {
			"focused" | "selected" => component.set_ring_color_from_entity_state(),
			_ => ()
		}
	}
}


