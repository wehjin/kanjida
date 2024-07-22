use wasm_bindgen::JsValue;
use web_sys::js_sys::Reflect;

pub struct JsKey(JsValue);

impl JsKey {
	pub fn key(&self) -> &JsValue { &self.0 }
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