use std::cell::RefCell;

use aframers::components::Position;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::Object;

use crate::js_sys_ex::JsKey;

thread_local! {
	pub static X_KEY: RefCell<JsKey> = RefCell::new("x".into());
	pub static Y_KEY: RefCell<JsKey> = RefCell::new("y".into());
	pub static Z_KEY: RefCell<JsKey> = RefCell::new("z".into());
}

pub struct StringSchemaProperty;
impl StringSchemaProperty {
	pub fn format(value: impl AsRef<str>) -> String {
		value.as_ref().to_string()
	}
	pub fn create_js(value: impl AsRef<str>) -> JsValue {
		JsValue::from(value.as_ref())
	}
	pub fn parse_js(data: &JsValue) -> String {
		data.as_string().unwrap()
	}
}

pub struct UsizeSchemaProperty;
impl UsizeSchemaProperty {
	pub fn format(value: usize) -> String {
		format!("{}", value)
	}
	pub fn create_js(value: usize) -> JsValue {
		JsValue::from(value)
	}
	pub fn parse_js(data: &JsValue) -> usize {
		let float = data.as_f64().unwrap();
		float as usize
	}
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

pub struct Vec3SchemaProperty;
impl Vec3SchemaProperty {
	pub fn format_float(x: f32, y: f32, z: f32) -> String {
		format!("{} {} {}", x, y, z)
	}
	pub fn try_position(data: &JsValue) -> Option<Position> {
		let x = X_KEY.with_borrow(|key| key.try_float(&data));
		let y = Y_KEY.with_borrow(|key| key.try_float(&data));
		let z = Z_KEY.with_borrow(|key| key.try_float(&data));
		if let (Some(x), Some(y), Some(z)) = (x, y, z) {
			Some(Position(x, y, z))
		} else {
			None
		}
	}
	pub fn js_from_position(Position(x, y, z): Position) -> JsValue {
		let object: JsValue = Object::new().unchecked_into();
		X_KEY.with_borrow(|key| key.set_float(&object, x));
		Y_KEY.with_borrow(|key| key.set_float(&object, y));
		Z_KEY.with_borrow(|key| key.set_float(&object, z));
		object
	}
}
impl Vec3SchemaProperty {
	pub fn format_usize(x: usize, y: usize, z: usize) -> String {
		format!("{} {} {}", x, y, z)
	}
	pub fn js_from_usize(x: usize, y: usize, z: usize) -> JsValue {
		let object: JsValue = Object::new().unchecked_into();
		X_KEY.with_borrow(|key| key.set_usize(&object, x));
		Y_KEY.with_borrow(|key| key.set_usize(&object, y));
		Z_KEY.with_borrow(|key| key.set_usize(&object, z));
		object
	}
	pub fn usize_from_js(data: &JsValue) -> (usize, usize, usize) {
		let x = X_KEY.with_borrow(|key| key.usize(&data));
		let y = Y_KEY.with_borrow(|key| key.usize(&data));
		let z = Z_KEY.with_borrow(|key| key.usize(&data));
		(x, y, z)
	}
}
