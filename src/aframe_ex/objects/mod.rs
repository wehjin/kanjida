use web_sys::js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

pub struct BuildObject(Object);

impl BuildObject {
	pub fn new() -> Self { Self(Object::new()) }
	pub fn to_object(self) -> Object { self.0 }
	pub fn set_property(self, name: impl AsRef<str>, value: &JsValue) -> Self {
		Reflect::set(&self.0, &name.as_ref().into(), value).expect("set property in Object");
		self
	}
	pub fn if_option_set_property(self, name: impl AsRef<str>, value: &Option<JsValue>) -> Self {
		if let Some(value) = value {
			self.set_property(name, value)
		} else {
			self
		}
	}
}