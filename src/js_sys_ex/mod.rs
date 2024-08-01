use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::{Object, Reflect};

pub struct JsKey(JsValue);

impl JsKey {
	pub fn key(&self) -> &JsValue { &self.0 }
	pub fn to_value(&self, object: &JsValue) -> JsValue {
		Reflect::get(object, self.key()).unwrap()
	}
}
impl JsKey {
	pub fn bool(&self, object: &JsValue) -> bool {
		self.to_value(object).as_bool().unwrap()
	}
	pub fn set_bool(&self, object: &JsValue, value: bool) {
		Reflect::set(object, self.key(), &value.into()).unwrap();
	}
}

impl JsKey {
	pub fn string(&self, object: &JsValue) -> String {
		self.to_value(object).as_string().unwrap()
	}
	pub fn set_string(&self, object: &JsValue, value: &str) {
		Reflect::set(object, self.key(), &value.into()).unwrap();
	}
}
impl JsKey {
	pub fn float(&self, object: &JsValue) -> f32 {
		self.to_value(object).as_f64().unwrap() as f32
	}
	pub fn set_float(&self, object: &JsValue, value: f32) {
		Reflect::set(object, self.key(), &value.into()).unwrap();
	}
}
impl JsKey {
	pub fn object(&self, object: &JsValue) -> Object {
		self.to_value(object).unchecked_into()
	}
	pub fn set_object(&self, object: &JsValue, value: &Object) {
		Reflect::set(object, self.key(), value.unchecked_ref()).unwrap();
	}
}
impl JsKey {
	pub fn usize(&self, object: &JsValue) -> usize {
		self.to_value(object).as_f64().unwrap() as usize
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