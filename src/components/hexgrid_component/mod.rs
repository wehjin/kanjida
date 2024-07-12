use aframers::af_sys::components::AComponent;
use aframers::components::core::ComponentValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::components::hexgrid_component::other::SelectedEntity;

pub mod handlers;
pub mod other;
pub mod registration;

pub const HEXGRID_COMPONENT_NAME: &str = "hexgrid";

pub enum Hexgrid { Enabled }

impl AsRef<str> for Hexgrid {
	fn as_ref(&self) -> &str {
		match self { Hexgrid::Enabled => "enabled" }
	}
}

impl ComponentValue for Hexgrid {
	fn component_name(&self) -> &str { HEXGRID_COMPONENT_NAME }
	fn component_value(&self) -> impl AsRef<str> { "enabled" }
}


#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AComponent)]
	pub type HexgridAComponent;
	#[wasm_bindgen(method, getter, js_name = extra_state)]
	pub fn selected_entity(this: &HexgridAComponent) -> SelectedEntity;
	#[wasm_bindgen(method, setter, js_name = extra_state)]
	pub fn set_selected_entity(this: &HexgridAComponent, value: SelectedEntity);
}
