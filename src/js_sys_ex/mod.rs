use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::{Object, Reflect};

pub struct JsKey(JsValue);

impl JsKey {
	pub fn key(&self) -> &JsValue { &self.0 }
}
impl JsKey {
	pub fn bool(&self, object: &JsValue) -> bool {
		let value = Reflect::get(object, self.key()).unwrap()
			.as_bool().unwrap()
			;
		value
	}
	pub fn set_bool(&self, object: &JsValue, value: bool) {
		Reflect::set(object, self.key(), &value.into()).unwrap();
	}
}

impl JsKey {
	pub fn string(&self, object: &JsValue) -> String {
		let value = Reflect::get(object, self.key()).unwrap()
			.as_string().unwrap()
			;
		value
	}
	pub fn set_string(&self, object: &JsValue, value: &str) {
		Reflect::set(object, self.key(), &value.into()).unwrap();
	}
}
impl JsKey {
	pub fn float(&self, object: &JsValue) -> f32 {
		let value = Reflect::get(object, self.key()).unwrap()
			.as_f64().unwrap()
			;
		value as f32
	}
	pub fn set_float(&self, object: &JsValue, value: f32) {
		Reflect::set(object, self.key(), &value.into()).unwrap();
	}
}
impl JsKey {
	pub fn object(&self, object: &JsValue) -> Object {
		let value = Reflect::get(object, self.key())
			.unwrap();
		value.unchecked_into()
	}
	pub fn set_object(&self, object: &JsValue, value: &Object) {
		Reflect::set(object, self.key(), value.unchecked_ref()).unwrap();
	}
}
impl JsKey {
	pub fn usize(&self, object: &JsValue) -> usize {
		let value = Reflect::get(object, self.key()).unwrap()
			.as_f64()
			.unwrap();
		value as usize
	}
	pub fn set_usize(&self, object: &JsValue, value: usize) {
		let js_value = value.into();
		Reflect::set(object, self.key(), &js_value).unwrap();
	}
}

impl<T: AsRef<str>> From<T> for JsKey {
	fn from(value: T) -> Self {
		let key = JsValue::from_str(value.as_ref());
		Self(key)
	}
}