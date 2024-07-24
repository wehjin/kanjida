use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
	pub type HexcellData;
	#[wasm_bindgen(method, getter)]
	pub fn glyph(this: &HexcellData) -> String;
	#[wasm_bindgen(method, getter)]
	pub fn status(this: &HexcellData) -> usize;
}
