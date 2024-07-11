use aframers::components::Color;
use aframers::entities::Entity;
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::material::Material;
use crate::components::hexcell_component;
use crate::components::hexcell_component::HexcellAComponent;

pub(super) fn handle_enter(this: HexcellAComponent, _event: JsValue) {
	let target = this.ring_entity();
	let material = Material::new().set_color(Color::Web("gold".into()));
	Entity::from(target).set_component(material).expect("set material");
}

pub(super) fn handle_leave(this: HexcellAComponent, _event: JsValue) {
	let ring_color = Color::Web(hexcell_component::api_ring_color(&this));
	let target = this.ring_entity();
	let material = Material::new().set_color(ring_color);
	Entity::from(target).set_component(material).expect("set material");
}

