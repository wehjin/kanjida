use aframers::components::core::ComponentSetting;
use aframers::components::Color;
use wasm_bindgen::JsValue;
use web_sys::js_sys::{Object, Reflect};

pub enum FieldType {
	Color,
	String,
	USize,
	Boolean,
}

impl FieldType {
	pub fn as_str(&self) -> &str {
		match self {
			FieldType::Color => "color",
			FieldType::String => "string",
			FieldType::USize => "int",
			FieldType::Boolean => "boolean",
		}
	}
}

pub struct Field(JsValue, FieldType);

impl Field {
	pub fn color(value: Color) -> Self {
		Self(value.as_setting_str().as_ref().into(), FieldType::Color)
	}
	pub fn string(s: impl AsRef<str>) -> Self {
		Self(JsValue::from_str(s.as_ref()), FieldType::String)
	}
	pub fn usize(value: usize) -> Self {
		Self(JsValue::from(value), FieldType::USize)
	}
	pub fn boolean(value: bool) -> Self {
		Self(JsValue::from(value), FieldType::Boolean)
	}
	pub fn to_object(self) -> Object {
		let object = Object::new();
		Reflect::set(&object, &"default".into(), &self.0).expect("set default");
		Reflect::set(&object, &"type".into(), &self.1.as_str().into()).expect("set type");
		object
	}
}