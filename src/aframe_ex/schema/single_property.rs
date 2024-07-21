use web_sys::js_sys::Object;
use crate::aframe_ex::schema::Schema;
use crate::aframe_ex::schema::fields::Field;

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