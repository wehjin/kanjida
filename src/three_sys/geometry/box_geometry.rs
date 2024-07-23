use wasm_bindgen::prelude::wasm_bindgen;

use crate::three_sys::BufferGeometry;
use crate::three_sys::geometry::geometry::Geometry;

#[wasm_bindgen(js_namespace = THREE)]
extern "C" {
	#[wasm_bindgen(extends = BufferGeometry, extends = Geometry)]
	pub type BoxGeometry;
	#[wasm_bindgen(constructor)]
	pub fn new() -> BoxGeometry;
}
