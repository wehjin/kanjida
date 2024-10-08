use std::f32::consts::PI;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::three_sys::BufferGeometry;
use crate::three_sys::geometry::geometry::Geometry;

#[wasm_bindgen(js_namespace = THREE)]
extern "C" {
	#[wasm_bindgen(extends = BufferGeometry, extends = Geometry)]
	pub type CircleGeometry;
	#[wasm_bindgen(constructor)]
	pub fn new(radius: f32, segments: u32, theta_start: f32, theta_length: f32) -> CircleGeometry;
	#[wasm_bindgen(constructor)]
	pub fn new_with_radius_and_segments(radius: f32, segments: u32) -> CircleGeometry;
}

pub const CIRCLE_RADIUS_DEFAULT: f32 = 1.;
pub const CIRCLE_SEGMENTS_DEFAULT: u32 = 32;
pub const CIRCLE_THETA_STATE_DEFAULT: f32 = 0.;
pub const CIRCLE_THETA_LENGTH_DEFAULT: f32 = 2. * PI;
pub fn circle_with_segments(value: u32) -> CircleGeometry {
	CircleGeometry::new(CIRCLE_RADIUS_DEFAULT, value, CIRCLE_THETA_STATE_DEFAULT, CIRCLE_THETA_LENGTH_DEFAULT)
}



