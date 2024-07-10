use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::components::Color;
use aframers::components::core::ComponentValue;
use aframers::entities::Entity;
use wasm_bindgen::{JsCast, JsValue};

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

fn a_entity_or_first_child(a_entity: AEntity) -> AEntity {
	a_entity.first_element_child()
		.map(|element| element.unchecked_into::<AEntity>())
		.unwrap_or_else(|| a_entity)
}
fn handle_enter(this: AComponent, _event: JsValue) {
	let target = a_entity_or_first_child(this.a_entity());
	let material = Material::new().set_color(Color::Web("gold".into()));
	Entity::from(target).set_component(material).expect("set material");
}

fn handle_leave(this: AComponent, _event: JsValue) {
	let target = a_entity_or_first_child(this.a_entity());
	let material = Material::new().set_color(Color::Web("silver".into()));
	Entity::from(target).set_component(material).expect("set material");
}

pub struct ColliderCheck;

impl ComponentValue for ColliderCheck {
	fn component_name(&self) -> &str { COLLIDER_CHECK }

	fn component_value(&self) -> impl AsRef<str> { "" }
}
