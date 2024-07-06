use aframers::component::Color;
use aframers::component::core::{Component, ComponentValue};
use aframers::entity::Entity;
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::cursor_component::CursorEvent::{MouseEnter, MouseLeave};
use crate::aframe_ex::components::material::Material;

pub const COLLIDER_CHECK: &str = "collider-check";

pub fn register() {
	let events = Events::new()
		.set_handler(MouseEnter, handle_enter)
		.set_handler(MouseLeave, handle_leave)
		;
	ComponentDefinition::new()
		.set_events(events)
		.register(COLLIDER_CHECK);
}
fn handle_enter(this: Component, _event: JsValue) {
	let element = this.el();
	let target = element.first_element_child().unwrap_or_else(|| element);
	let material = Material::new().set_color(Color::Web("gold"));
	Entity(target).set_component(material).expect("set material");
}

fn handle_leave(this: Component, _event: JsValue) {
	let element = this.el();
	let target = element.first_element_child().unwrap_or_else(|| element);
	let material = Material::new().set_color(Color::Web("silver"));
	Entity(target).set_component(material).expect("set material");
}

pub struct ColliderCheck;

impl ComponentValue for ColliderCheck {
	fn component_name(&self) -> &str { COLLIDER_CHECK }

	fn component_value(&self) -> impl AsRef<str> { "" }
}
