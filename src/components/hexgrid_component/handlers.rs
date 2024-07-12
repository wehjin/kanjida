use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::CustomEvent;

use crate::aframe_ex::events::StateEvent;
use crate::components::hexgrid_component::HexgridAComponent;

pub fn handle_state_added(component: AComponent, event: JsValue) {
	let component = component.unchecked_into::<HexgridAComponent>();
	let custom = event.unchecked_into::<CustomEvent>();
	if let Some(state_added) = StateEvent::try_added(&custom) {
		match state_added.state() {
			"selected" => {
				let target = custom.target().unwrap().unchecked_into::<AEntity>();
				component.update_selected_entity_notifying_old(&target);
			}
			_ => ()
		}
	}
}

pub fn handle_state_removed(component: AComponent, event: JsValue) {
	let component = component.unchecked_into::<HexgridAComponent>();
	let custom = event.unchecked_into::<CustomEvent>();
	if let Some(state_removed) = StateEvent::try_removed(&custom) {
		match state_removed.state() {
			"selected" => {
				let target = custom.target().unwrap().unchecked_into::<AEntity>();
				component.update_unselected_entity(&target);
			}
			_ => ()
		}
	}
}