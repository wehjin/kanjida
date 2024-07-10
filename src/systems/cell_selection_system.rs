use aframers::browser::log;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;

use crate::aframe_ex::systems::{aframe_system_def, register_system, System};

#[wasm_bindgen]
pub struct CellSelectionSystem {
	selected: Option<Element>,
}

#[wasm_bindgen]
impl CellSelectionSystem {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self { selected: None }
	}
	pub fn __system_init(&self, _system: &System) {
		log("cell-selection-system init")
	}
	pub fn __system_select_cell(&mut self, _system: &System, cell: &Element) {
		self.selected = Some(cell.clone());
	}
	pub fn __system_selected(&self, _system: &System) -> Option<Element> {
		self.selected.clone()
	}
}
pub fn register() {
	let definition = aframe_system_def(CellSelectionSystem::new().into());
	register_system("cell-selection", &definition);
}
