use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace = THREE)]
extern "C" {
	#[derive(Clone)]
	pub type Geometry;
}
