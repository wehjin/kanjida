use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::components::Color;
use aframers::entities::Entity;
use wasm_bindgen::{JsCast, JsValue};

use crate::aframe_ex::components::material::Material;
use crate::components::hexcell_component;

pub(super) fn handle_enter(this: AComponent, _event: JsValue) {
	let target = ring_entity_in_hexcell(this);
	let material = Material::new().set_color(Color::Web("gold".into()));
	Entity::from(target).set_component(material).expect("set material");
}

pub(super) fn handle_leave(this: AComponent, _event: JsValue) {
	let ring_color = Color::Web(hexcell_component::api_ring_color(&this));
	let target = ring_entity_in_hexcell(this);
	let material = Material::new().set_color(ring_color);
	Entity::from(target).set_component(material).expect("set material");
}

fn ring_entity_in_hexcell(hexcell: AComponent) -> AEntity {
	hexcell.a_entity().first_element_child().expect("ring element").unchecked_into::<AEntity>()
}

