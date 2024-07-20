use aframers::components::Color;
use aframers::components::core::ComponentValue;
use wasm_bindgen::JsValue;
use web_sys::js_sys::{Object, Reflect};

pub trait Schema {
	fn to_object(self) -> Object;
}

#[must_use]
pub struct SinglePropertySchema(Object);
impl<T> From<T> for SinglePropertySchema
where
	T: AsRef<str> + Sized,
{
	fn from(value: T) -> Self {
		Self::from(Field::string(value))
	}
}
impl From<Field> for SinglePropertySchema {
	fn from(value: Field) -> Self {
		Self(value.to_object())
	}
}
impl Schema for SinglePropertySchema {
	fn to_object(self) -> Object { self.0 }
}

#[must_use]
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
	Color,
	String,
	USize,
}

impl FieldKind {
	pub fn as_str(&self) -> &str {
		match self {
			FieldKind::Color => "color",
			FieldKind::String => "string",
			FieldKind::USize => "int",
		}
	}
}

pub struct Field(JsValue, FieldKind);
impl Field {
	pub fn color(value: Color) -> Self {
		Self(value.component_value().as_ref().into(), FieldKind::Color)
	}
	pub fn string(s: impl AsRef<str>) -> Self {
		Self(JsValue::from_str(s.as_ref()), FieldKind::String)
	}
	pub fn usize(value: usize) -> Self {
		Self(JsValue::from(value), FieldKind::USize)
	}
	pub fn to_object(self) -> Object {
		let object = Object::new();
		Reflect::set(&object, &"default".into(), &self.0).expect("set default");
		Reflect::set(&object, &"type".into(), &self.1.as_str().into()).expect("set type");
		object
	}
}