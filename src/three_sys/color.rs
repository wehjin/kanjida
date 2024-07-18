use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace = THREE)]
extern "C" {
	#[wasm_bindgen]
	pub type Color;
	#[wasm_bindgen(constructor)]
	pub fn new() -> Color;
	#[wasm_bindgen(constructor)]
	pub fn new_rgb(r: f32, g: f32, b: f32) -> Color;
	#[wasm_bindgen(constructor)]
	pub fn new_str(s: &str) -> Color;
	#[wasm_bindgen(method, js_name = setRGB)]
	pub fn set_rgb(this: &Color, r: f32, g: f32, b: f32) -> Color;
}

