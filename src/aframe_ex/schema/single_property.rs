use wasm_bindgen::JsValue;
use web_sys::js_sys::{Object, Reflect};

use crate::aframe_ex::schema::{Schema, SchemaProperty, SchemaPropertyType};
use crate::aframe_ex::schema::fields::Field;

#[must_use]
pub struct SinglePropertySchema(Object);

impl SinglePropertySchema {
	pub fn new_with_default(type_: SchemaPropertyType, default: JsValue) -> Self {
		let object = Object::new();
		Reflect::set(&object, &"default".into(), &default).unwrap();
		Reflect::set(&object, &"type".into(), &type_.as_str().into()).unwrap();
		Self(object)
	}
}
impl Schema for SinglePropertySchema {
	fn to_object(self) -> Object { self.0 }
}
impl<T: SchemaProperty> From<T> for SinglePropertySchema {
	fn from(value: T) -> Self {
		let default = value.to_schema_property_default();
		let type_ = value.to_schema_property_type();
		Self::new_with_default(type_, default)
	}
}

impl From<Field> for SinglePropertySchema {
	fn from(value: Field) -> Self {
		Self(value.to_object())
	}
}

