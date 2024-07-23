use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::{Function, Object};

#[wasm_bindgen(module = "three/addons/loaders/FontLoader.js")]
extern "C" {
	pub type FontLoader;
	#[wasm_bindgen(constructor)]
	pub fn new() -> FontLoader;
	#[wasm_bindgen(method)]
	pub fn load(this: &FontLoader, url: &str, on_load: &Function) -> Object;
}
