use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Array;

use crate::three_sys::geometry::geometry::Geometry;

#[wasm_bindgen(js_namespace = THREE)]
extern "C" {
	#[wasm_bindgen(extends = Geometry)]
	pub type BufferGeometry;
	#[wasm_bindgen(constructor)]
	pub fn new() -> BufferGeometry;
	#[wasm_bindgen(method)]
	pub fn translate(this: &BufferGeometry, x: f32, y: f32, z: f32) -> BufferGeometry;
	#[wasm_bindgen(method)]
	pub fn scale(this: &BufferGeometry, x: f32, y: f32, z: f32) -> BufferGeometry;
}

#[wasm_bindgen(module = "three/addons/utils/BufferGeometryUtils.js")]
extern "C" {
	#[wasm_bindgen(js_name = mergeGeometries)]
	pub fn merge_geometries(geometries: &Array, use_groups: bool) -> BufferGeometry;
}
