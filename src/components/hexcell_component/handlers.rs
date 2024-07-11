use aframers::components::Color;
use aframers::entities::Entity;
use wasm_bindgen::{JsCast, JsValue};

use crate::aframe_ex::components::material::Material;
use crate::components::hexcell_component;
use crate::components::hexcell_component::HexcellAComponent;
use crate::systems::hexcell_system;
use crate::systems::hexcell_system::HexcellASystem;

pub(super) fn handle_enter(this: HexcellAComponent, _event: JsValue) {
	let target = this.ring_entity();
	let material = Material::new().set_color(Color::Web("gold".into()));
	Entity::from(target).set_component(material).expect("set material");
}

pub(super) fn handle_leave(this: HexcellAComponent, _event: JsValue) {
	this.restore_ring_color();
}

pub(super) fn handle_click(this: HexcellAComponent, _event: JsValue) {
	let system = this.get_system(hexcell_system::NAME).unchecked_into::<HexcellASystem>();
	let was_selected = system.select_cell(&this.a_entity());
	if let Some(was_selected) = was_selected {
		let hexcell = was_selected.get_component(hexcell_component::NAME).unchecked_into::<HexcellAComponent>();
		hexcell.restore_ring_color();
	}
}



