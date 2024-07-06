use aframers::component::Color;
use aframers::component::core::{Component, ComponentValue};
use aframers::entity::Entity;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;
use web_sys::js_sys::Reflect;

use crate::aframe_ex::components::core::ComponentDefinition;
use crate::aframe_ex::components::cursor_component::CursorEvent;
use crate::aframe_ex::components::material::Material;
use crate::aframe_ex::events::core::EventListener;

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
	_enter: EventListener,
	_leave: EventListener,
	target: Entity,
}
#[wasm_bindgen]
impl ColliderCheckRust {
	#[wasm_bindgen(js_name = getTarget)]
	pub fn get_target(&self) -> Element {
		self.target.element().clone()
	}
}

impl ColliderCheckRust {
	pub fn start(entity: Entity) -> Self {
		let target = if let Some(child) = entity.element().first_element_child() {
			Entity(child)
		} else {
			entity.clone()
		};
		let enter = EventListener::new({
			let target = target.clone();
			move |_| {
				let material = Material::new().set_color(Color::Web("gold"));
				target.clone().set_component(material).expect("set material");
			}
		});
		let leave = EventListener::new({
			let target = target.clone();
			move |_| {
				let material = Material::new().set_color(Color::Web("silver"));
				target.clone().set_component(material).expect("set material");
			}
		});

		entity.element().add_event_listener_with_callback(
			CursorEvent::MouseEnter.as_str(),
			enter.as_function(),
		).expect("add enter listener");

		entity.element().add_event_listener_with_callback(
			CursorEvent::MouseLeave.as_str(),
			leave.as_function(),
		).expect("add leave listener");

		Self {
			_enter: enter,
			_leave: leave,
			target,
		}
	}
}
