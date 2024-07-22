use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::{Object, Reflect};

pub struct Vec2SchemaProperty {}
impl Vec2SchemaProperty {
	pub fn format(x: usize, y: usize) -> String {
		format!("{} {}", x, y)
	}
	pub fn create_js(x: usize, y: usize) -> JsValue {
		let object: JsValue = Object::new().unchecked_into();
		Reflect::set(&object, &"x".into(), &x.into()).unwrap();
		Reflect::set(&object, &"y".into(), &y.into()).unwrap();
		object
	}
}

