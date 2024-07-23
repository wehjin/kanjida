use std::cell::RefCell;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Object;

use crate::js_sys_ex::JsKey;
use crate::three_sys::{BufferGeometry, Geometry};

#[wasm_bindgen(module = "three/addons/geometries/TextGeometry.js")]
extern "C" {
	#[wasm_bindgen(extends = BufferGeometry, extends = Geometry)]
	pub type TextGeometry;
	#[wasm_bindgen(constructor)]
	pub fn new(text: &str, parameters: &Object) -> TextGeometry;
}

thread_local! {
	pub static DEPTH_KEY: RefCell<JsKey> = RefCell::new("depth".into());
	pub static FONT_KEY: RefCell<JsKey> = RefCell::new("font".into());
	pub static SIZE_KEY: RefCell<JsKey> = RefCell::new("size".into());
}

pub struct TextGeometryParameters(Object);

impl TextGeometryParameters {
	pub fn new() -> Self { Self(Object::new()) }
	pub fn as_js(&self) -> &Object {
		self.0.unchecked_ref()
	}
	pub fn set_font(&self, value: &Object) {
		FONT_KEY.with_borrow(|key| {
			key.set_object(self.as_js(), value);
		});
	}
	pub fn set_size(&self, value: f32) {
		SIZE_KEY.with_borrow(|key| {
			key.set_float(self.as_js(), value);
		});
	}
	pub fn set_depth(&self, value: f32) {
		DEPTH_KEY.with_borrow(|key| {
			key.set_float(self.as_js(), value);
		});
	}
}
