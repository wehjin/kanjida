use web_sys::js_sys::{Object, Reflect};

use crate::aframe_ex::schema::fields::Field;
use crate::aframe_ex::schema::Schema;

#[must_use]
pub struct MultiPropertySchema(Object);

impl MultiPropertySchema {
	pub fn new() -> Self {
		Self(Object::new())
	}
	pub fn push(self, name: impl AsRef<str>, field: Field) -> Self {
		let property_name = name.as_ref();
		let property_def = field.to_object();
		Reflect::set(&self.0, &property_name.into(), &property_def).expect("set field");
		self
	}
}

impl Schema for MultiPropertySchema {
	fn to_object(self) -> Object {
		self.0
	}
}