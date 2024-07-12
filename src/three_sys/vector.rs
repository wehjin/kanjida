use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
	pub type Vector3;
	#[wasm_bindgen(constructor, js_namespace = THREE)]
	pub fn new(x: f32, y: f32, z: f32) -> Vector3;
	#[wasm_bindgen(constructor, js_namespace = THREE)]
	pub fn origin() -> Vector3;
	#[wasm_bindgen(method, getter)]
	pub fn x(this: &Vector3) -> f32;
	#[wasm_bindgen(method, getter)]
	pub fn y(this: &Vector3) -> f32;
	#[wasm_bindgen(method, getter)]
	pub fn z(this: &Vector3) -> f32;

	#[wasm_bindgen(method, js_name = setX)]
	pub fn set_x(this: &Vector3, value: f32) -> Vector3;
	#[wasm_bindgen(method, js_name = setY)]
	pub fn set_y(this: &Vector3, value: f32) -> Vector3;
	#[wasm_bindgen(method, js_name = setZ)]
	pub fn set_z(this: &Vector3, value: f32) -> Vector3;
}


