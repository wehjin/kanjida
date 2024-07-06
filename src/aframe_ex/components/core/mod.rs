use aframers::component::core::{Component, register_component};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use web_sys::js_sys::{Array, Object, Reflect};

use crate::aframe_ex::{js, Schema};

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
	pub fn set_init(self, value: impl Fn(Component) + 'static) -> Self {
		let closure = Closure::wrap(Box::new(value) as Box<dyn Fn(Component)>);
		let new_self = self.set_property("init", &js::to_init(&closure));
		closure.forget();
		new_self
	}
	pub fn set_property(self, name: impl AsRef<str>, value: &JsValue) -> Self {
		Reflect::set(&self.0, &name.as_ref().into(), &value).expect("set property");
		self
	}
}