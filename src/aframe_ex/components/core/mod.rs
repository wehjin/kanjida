use aframers::af_sys::components::{AComponent, register_component};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::convert::FromWasmAbi;
use web_sys::js_sys::{Array, Object, Reflect};

use crate::aframe_ex::{js, Schema};
use crate::aframe_ex::js::bind_this_to_component;
use crate::aframe_ex::scenes::Scene;
use crate::aframe_ex::systems::ASystem;

pub struct Events(Object);

impl Events {
	pub fn new() -> Self {
		Self(Object::new())
	}
	pub fn to_object(self) -> Object {
		self.0
	}
	pub fn set_handler<T>(self, event_name: impl AsRef<str>, handler: impl Fn(T, JsValue) + 'static) -> Self
	where
		T: AsRef<AComponent> + FromWasmAbi + 'static,
	{
		let bound_handler = bind_this_to_component(handler);
		Reflect::set(&self.0, &event_name.as_ref().into(), &bound_handler).expect("set handler");
		self
	}
}

pub struct Dependencies(Array);

impl Dependencies {
	pub fn new(component_name: impl AsRef<str>) -> Self {
		let array = Array::new_with_length(1);
		array.set(0, component_name.as_ref().into());
		Self(array)
	}
	pub fn to_array(self) -> Array { self.0 }
}

pub struct ComponentDefinition(Object);

impl ComponentDefinition {
	pub fn new() -> Self {
		ComponentDefinition(Object::new())
	}
	pub fn register(self, name: impl AsRef<str>) {
		register_component(name.as_ref(), &self.0);
	}
	pub fn set_dependencies(self, dependencies: Dependencies) -> Self {
		self.set_property("dependencies", &dependencies.to_array())
	}
	pub fn set_schema(self, schema: Schema) -> Self {
		self.set_property("schema", &schema.to_object())
	}
	pub fn set_events(self, events: Events) -> Self {
		self.set_property("events", &events.to_object())
	}
	pub fn set_init(self, value: impl Fn(AComponent) + 'static) -> Self {
		let closure = Closure::wrap(Box::new(value) as Box<dyn Fn(AComponent)>);
		let new_self = self.set_property("init", &js::to_init(&closure));
		closure.forget();
		new_self
	}
	pub fn set_property(self, name: impl AsRef<str>, value: &JsValue) -> Self {
		Reflect::set(&self.0, &name.as_ref().into(), &value).expect("set property");
		self
	}
}

pub fn component_get_system(a_component: &AComponent, system_name: impl AsRef<str>) -> ASystem {
	let scene = Scene::from(a_component.a_entity().a_scene());
	let a_system = scene.a_system(system_name.as_ref());
	a_system
}

pub fn component_get_system_into<T: JsCast>(a_component: &AComponent, system_name: impl AsRef<str>) -> T {
	component_get_system(a_component, system_name).unchecked_into()
}

pub fn component_get_data_into<T: JsCast>(a_component: &AComponent) -> T {
	a_component.data().unchecked_into::<T>()
}