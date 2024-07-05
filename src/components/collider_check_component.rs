use aframers::component::Color;
use aframers::component::core::{Component, ComponentValue};
use aframers::entity::Entity;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::{Closure, wasm_bindgen};
use web_sys::js_sys::Reflect;

use crate::aframe_ex::ComponentDefinition;
use crate::aframe_ex::components::material::Material;
use crate::aframe_ex::components::raycaster_component::RayCasterEvent;

pub const COLLIDER_CHECK: &str = "collider-check";


pub struct ColliderCheck;
impl ComponentValue for ColliderCheck {
	fn component_name(&self) -> &str { COLLIDER_CHECK }

	fn component_value(&self) -> impl AsRef<str> { "" }
}

pub fn register() {
	ComponentDefinition::new()
		.set_init(|this: Component| {
			let entity = Entity(this.el());
			let context = ColliderCheckRust::start(entity);
			Reflect::set(&this, &"colliderCheckRust".into(), &context.into()).expect("set context");
		})
		.register(COLLIDER_CHECK);
}

#[wasm_bindgen]
pub struct ColliderCheckRust {
	_on_intersected: Closure<dyn FnMut()>,
	_on_intersected_cleared: Closure<dyn FnMut()>,
}

impl ColliderCheckRust {
	pub fn start(entity: Entity) -> Self {
		let on_intersected = {
			let target = entity.clone();
			Closure::new(move || {
				let material = Material::new().set_color(Color::Web("gold"));
				target.clone().set_component(material).expect("set material");
			})
		};
		entity.element().add_event_listener_with_callback(
			RayCasterEvent::Intersected.as_str(),
			on_intersected.as_ref().unchecked_ref(),
		).expect("add intersected listener");

		let on_intersected_cleared = {
			let target = entity.clone();
			Closure::new(move || {
				let material = Material::new().set_color(Color::Web("silver"));
				target.clone().set_component(material).expect("set material");
			})
		};
		entity.element().add_event_listener_with_callback(
			RayCasterEvent::IntersectedCleared.as_str(),
			on_intersected_cleared.as_ref().unchecked_ref(),
		).expect("add intersected-cleared listener");

		Self { _on_intersected: on_intersected, _on_intersected_cleared: on_intersected_cleared }
	}
}
