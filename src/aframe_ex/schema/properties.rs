use aframers::components::Color;
use aframers::components::core::ComponentValue;
use wasm_bindgen::JsValue;
use web_sys::js_sys::{Object, Reflect};

pub enum PropertyType {
	Color,
	String,
	USize,
}

impl PropertyType {
	pub fn as_str(&self) -> &str {
		match self {
			PropertyType::Color => "color",
			PropertyType::String => "string",
			PropertyType::USize => "int",
		}
	}
}

pub struct Field(JsValue, PropertyType);

impl Field {
	pub fn color(value: Color) -> Self {
		Self(value.component_value().as_ref().into(), PropertyType::Color)
	}
	pub fn string(s: impl AsRef<str>) -> Self {
		Self(JsValue::from_str(s.as_ref()), PropertyType::String)
	}
	pub fn usize(value: usize) -> Self {
		Self(JsValue::from(value), PropertyType::USize)
	}
	pub fn to_object(self) -> Object {
		let object = Object::new();
		Reflect::set(&object, &"default".into(), &self.0).expect("set default");
		Reflect::set(&object, &"type".into(), &self.1.as_str().into()).expect("set type");
		object
	}
}