use aframers::components::Color;
use aframers::components::core::ComponentValue;
use wasm_bindgen::JsValue;
use web_sys::js_sys::{Object, Reflect};

pub trait Schema {
	fn to_object(self) -> Object;
}

pub struct SinglePropertySchema(Object);

impl From<Field> for SinglePropertySchema {
	fn from(value: Field) -> Self {
		Self(value.to_object())
	}
}

impl Schema for SinglePropertySchema {
	fn to_object(self) -> Object { self.0 }
}

pub struct MultiPropertySchema(Object);

impl MultiPropertySchema {
	pub fn new() -> Self {
		Self(Object::new())
	}
	pub fn push(self, name: impl AsRef<str>, field: Field) -> Self {
		Reflect::set(&self.0, &name.as_ref().into(), &field.to_object()).expect("set field");
		self
	}
}

impl Schema for MultiPropertySchema {
	fn to_object(self) -> Object {
		self.0
	}
}

pub enum FieldKind {
	String,
	Color,
}

impl FieldKind {
	pub fn as_str(&self) -> &str {
		match self {
			FieldKind::String => "string",
			FieldKind::Color => "color",
		}
	}
}

pub struct Field(JsValue, FieldKind);

impl Field {
	pub fn string(s: impl AsRef<str>) -> Self {
		Self(JsValue::from_str(s.as_ref()), FieldKind::String)
	}
	pub fn color(value: Color) -> Self {
		Self(value.component_value().as_ref().into(), FieldKind::Color)
	}
	pub fn to_object(self) -> Object {
		let object = Object::new();
		Reflect::set(&object, &"default".into(), &self.0).expect("set default");
		Reflect::set(&object, &"type".into(), &self.1.as_str().into()).expect("set type");
		object
	}
}