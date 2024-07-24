use wasm_bindgen::prelude::wasm_bindgen;

use crate::three_sys::BufferGeometry;
use crate::three_sys::geometry::geometry::Geometry;

#[wasm_bindgen(js_namespace = THREE)]
extern "C" {
	#[wasm_bindgen(extends = BufferGeometry, extends = Geometry)]
	pub type RingGeometry;
	#[wasm_bindgen(constructor)]
	pub fn new(
		inner_radius: f32, outer_radius: f32,
		theta_segments: usize, phi_segments: usize,
		theta_start: f32, theta_length: f32,
	) -> RingGeometry;
	#[wasm_bindgen(constructor)]
	pub fn new_with_radius_and_segments(inner_radius: f32, outer_radius: f32, theta_segments: usize) -> RingGeometry;
}
