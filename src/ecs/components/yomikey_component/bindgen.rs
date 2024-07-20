use aframers::af_sys::components::AComponent;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::ecs::components::yomikey_component::yk_state::YkeyState;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AComponent)]
	pub type YomikeyAComponent;
	#[wasm_bindgen(method, getter, js_name = rustState)]
	pub fn take_rust_state(this: &YomikeyAComponent) -> YkeyState;
	#[wasm_bindgen(method, setter, js_name = rustState)]
	pub fn set_rust_state(this: &YomikeyAComponent, value: YkeyState);
	#[wasm_bindgen(method, getter, js_name = data)]
	pub fn yomi_code(this: &YomikeyAComponent) -> usize;
}


