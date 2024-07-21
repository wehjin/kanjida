use web_sys::js_sys::{Object, Reflect};
use crate::aframe_ex::schema::Schema;
use crate::aframe_ex::schema::properties::Field;

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