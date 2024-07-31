use std::cell::{LazyCell, RefCell};

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
	pub static CURVE_SEGMENTS_KEY: RefCell<JsKey> = RefCell::new("curveSegments".into());
	pub static BEVEL_ENABLED_KEY: LazyCell<JsKey> = LazyCell::new(||"bevelEnabled".into());
	pub static BEVEL_THICKNESS_KEY: LazyCell<JsKey> = LazyCell::new(||"bevelThickness".into());
	pub static BEVEL_SIZE_KEY: LazyCell<JsKey> = LazyCell::new(||"bevelSize".into());
	pub static BEVEL_OFFSET_KEY: LazyCell<JsKey> = LazyCell::new(||"bevelOffset".into());
	pub static BEVEL_SEGMENTS_KEY: LazyCell<JsKey> = LazyCell::new(||"bevelSegments".into());
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
	pub fn set_curve_segments(&self, value: usize) {
		CURVE_SEGMENTS_KEY.with_borrow(|key| {
			key.set_usize(self.as_js(), value)
		})
	}
	pub fn set_bevel_enabled(&self, value: bool) {
		BEVEL_ENABLED_KEY.with(|key| {
			key.set_bool(self.as_js(), value)
		})
	}
	pub fn set_bevel_thickness(&self, value: f32) {
		BEVEL_THICKNESS_KEY.with(|key| {
			key.set_float(self.as_js(), value)
		})
	}
	pub fn set_bevel_size(&self, value: f32) {
		BEVEL_SIZE_KEY.with(|key| {
			key.set_float(self.as_js(), value)
		})
	}
	pub fn set_bevel_offset(&self, value: f32) {
		BEVEL_OFFSET_KEY.with(|key| {
			key.set_float(self.as_js(), value)
		})
	}
	pub fn set_bevel_segments(&self, value: usize) {
		BEVEL_SEGMENTS_KEY.with(|key| {
			key.set_usize(self.as_js(), value)
		})
	}
}
