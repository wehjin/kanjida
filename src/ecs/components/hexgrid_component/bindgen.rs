use aframers::af_sys::components::AComponent;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::ecs::components::hexgrid_component::other::SelectedEntity;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AComponent)]
	pub type HexgridAComponent;
	#[wasm_bindgen(method, getter, js_name = extra_state)]
	pub fn selected_entity(this: &HexgridAComponent) -> SelectedEntity;
	#[wasm_bindgen(method, setter, js_name = extra_state)]
	pub fn set_selected_entity(this: &HexgridAComponent, value: SelectedEntity);
}
