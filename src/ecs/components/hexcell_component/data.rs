use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
	pub type HexcellSettings;
	#[wasm_bindgen(method, getter)]
	pub fn glyph(this: &HexcellSettings) -> String;
	#[wasm_bindgen(method, getter)]
	pub fn solved(this: &HexcellSettings) -> bool;
}
