use std::cell::RefCell;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::Object;

use crate::js_sys_ex::JsKey;

thread_local! {
	pub static X_KEY: RefCell<JsKey> = RefCell::new("x".into());
	pub static Y_KEY: RefCell<JsKey> = RefCell::new("y".into());
}

pub struct Vec2SchemaProperty;
impl Vec2SchemaProperty {
	pub fn format(x: usize, y: usize) -> String {
		format!("{} {}", x, y)
	}
	pub fn create_js(x: usize, y: usize) -> JsValue {
		let object: JsValue = Object::new().unchecked_into();
		X_KEY.with_borrow(|key| key.set_usize(&object, x));
		Y_KEY.with_borrow(|key| key.set_usize(&object, y));
		object
	}
	pub fn parse_js(data: &JsValue) -> (usize, usize) {
		let x = X_KEY.with_borrow(|key| key.usize(&data));
		let y = Y_KEY.with_borrow(|key| key.usize(&data));
		(x, y)
	}
}

