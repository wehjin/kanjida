use wasm_bindgen::prelude::wasm_bindgen;

use crate::three_sys::geometry::geometry::Geometry;

#[wasm_bindgen(js_namespace = THREE)]
extern "C" {
	#[wasm_bindgen(extends = Geometry)]
	pub type BufferGeometry;
	#[wasm_bindgen(constructor)]
	pub fn new() -> BufferGeometry;
}


