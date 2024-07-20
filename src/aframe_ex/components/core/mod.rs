use aframers::af_sys::components::{AComponent, register_component};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi, RefFromWasmAbi};
use web_sys::js_sys::{Array, Function, Object, Reflect};

use crate::aframe_ex::js;
use crate::aframe_ex::js::{aframers_bind_init_with_extra_state, aframers_bind_other_with_extra_state, aframers_bind_remove_with_extra_state, bind_this_to_component};
use crate::aframe_ex::schema::Schema;

pub mod properties {
	use aframers::components::core::{ComponentValue, ToPropertyValue};

	pub trait ComponentProperty: AsPropertyName + ToPropertyValue {
		fn to_attribute_string(&self) -> String {
			format!("{}: {}", self.as_property_name(), self.to_property_value())
		}
	}
	pub trait AsPropertyName {
		fn as_property_name(&self) -> &str;
	}

	impl<T: ComponentValue> AsPropertyName for T {
		fn as_property_name(&self) -> &str {
			self.component_name()
		}
	}

	pub struct MultiPropertyAttributeValue(Vec<String>);
	impl MultiPropertyAttributeValue {
		pub fn new() -> Self {
			Self(Vec::new())
		}
		pub fn add_property_value(self, name: impl AsRef<str>, value: &Option<impl ToPropertyValue>) -> Self {
			if let Some(value) = value {
				let mut vec = self.0;
				vec.push(format!("{}: {}", name.as_ref(), value.to_property_value()));
				Self(vec)
			} else {
				self
			}
		}
		pub fn add_property(self, property: &Option<impl ComponentProperty>) -> Self {
			if let Some(value) = property {
				let mut vec = self.0;
				vec.push(value.to_attribute_string());
				Self(vec)
			} else {
				self
			}
		}
		pub fn to_attribute_value(self) -> String { self.0.join("; ") }
	}
}

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

#[must_use]
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
	pub fn set_schema(self, schema: impl Schema) -> Self {
		self.set_property("schema", &schema.to_object())
	}
	pub fn set_events(self, events: Events) -> Self {
		self.set_property("events", &events.to_object())
	}
	pub fn set_init_update_remove<T, U>(
		self,
		init: impl Fn(&T) -> U + 'static,
		update: impl Fn(&T) + 'static,
		remove: impl Fn(&T) + 'static,
	) -> Self
	where
		T: AsRef<AComponent> + RefFromWasmAbi + 'static,
		U: IntoWasmAbi + 'static,
	{
		let bound_init = aframers_bind_init_with_extra_state(function_from_component_fn_with_return(init));
		let bound_update = aframers_bind_other_with_extra_state(function_from_component_fn(update));
		let bound_remove = aframers_bind_remove_with_extra_state(function_from_component_fn(remove));
		self.set_property("init", &bound_init)
			.set_property("update", &bound_update)
			.set_property("remove", &bound_remove)
	}

	pub fn set_init_remove_ref<T, U>(self, init: impl Fn(&T) -> U + 'static, remove: impl Fn(&T) + 'static) -> Self
	where
		T: AsRef<AComponent> + RefFromWasmAbi + 'static,
		U: IntoWasmAbi + 'static,
	{
		let bound_init = aframers_bind_init_with_extra_state(function_from_component_fn_with_return(init));
		let bound_remove = aframers_bind_remove_with_extra_state(function_from_component_fn(remove));
		self.set_property("init", &bound_init).set_property("remove", &bound_remove)
	}

	pub fn set_init_remove_with_extra_state<T, U>(
		self,
		init: impl Fn(T) -> U + 'static,
		remove: impl Fn(T) + 'static,
	) -> Self
	where
		T: AsRef<AComponent> + FromWasmAbi + 'static,
		U: IntoWasmAbi + 'static,
	{
		let bound_init = {
			let unbound = Closure::wrap(Box::new(init) as Box<dyn Fn(T) -> U>).into_js_value().unchecked_into::<Function>();
			aframers_bind_init_with_extra_state(unbound)
		};
		let bound_remove = {
			let unbound = Closure::wrap(Box::new(remove) as Box<dyn Fn(T)>).into_js_value().unchecked_into::<Function>();
			aframers_bind_remove_with_extra_state(unbound)
		};
		self.set_property("init", &bound_init).set_property("remove", &bound_remove)
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

fn function_from_component_fn_with_return<T, U>(f: impl Fn(&T) -> U + Sized + 'static) -> Function
where
	T: AsRef<AComponent> + RefFromWasmAbi + 'static,
	U: IntoWasmAbi + 'static,
{
	Closure::wrap(Box::new(f) as Box<dyn Fn(&T) -> U>).into_js_value().unchecked_into::<Function>()
}

fn function_from_component_fn<T>(f: impl Fn(&T) + Sized + 'static) -> Function
where
	T: AsRef<AComponent> + RefFromWasmAbi + 'static,
{
	Closure::wrap(Box::new(f) as Box<dyn Fn(&T)>).into_js_value().unchecked_into::<Function>()
}
