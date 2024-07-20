use aframers::af_sys::components::AComponent;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AComponent)]
	pub type YomigunAComponent;
	#[wasm_bindgen(method, getter, js_name = data)]
	pub fn yomigun_data(this: &YomigunAComponent) -> YomigunAData;
}

#[wasm_bindgen]
extern "C" {
	pub type YomigunAData;
	#[wasm_bindgen(method, getter, js_name = yomiCode)]
	pub fn yomi_code(this: &YomigunAData) -> usize;
}
