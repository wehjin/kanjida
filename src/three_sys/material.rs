use wasm_bindgen::prelude::wasm_bindgen;

use crate::three_sys::color::Color;

#[wasm_bindgen(js_namespace = THREE)]
extern "C" {
	pub type Material;
	#[wasm_bindgen(constructor)]
	pub fn new() -> Material;
	#[wasm_bindgen(method, getter)]
	pub fn name(this: &Material) -> String;
	#[wasm_bindgen(method, setter)]
	pub fn set_name(this: &Material, value: &str);
	#[wasm_bindgen(method, getter, js_name = needsUpdate)]
	pub fn needs_update(this: &Material) -> bool;
	#[wasm_bindgen(method, setter, js_name = needsUpdate)]
	pub fn set_needs_update(this: &Material, value: bool);
}

#[wasm_bindgen(js_namespace = THREE)]
extern "C" {
	#[wasm_bindgen(extends = Material)]
	pub type MeshBasicMaterial;
	#[wasm_bindgen(constructor)]
	pub fn new() -> MeshBasicMaterial;
	#[wasm_bindgen(method, getter)]
	pub fn color(this: &MeshBasicMaterial) -> Color;
	#[wasm_bindgen(method, setter)]
	pub fn set_color(this: &MeshBasicMaterial, color: &Color);
}


