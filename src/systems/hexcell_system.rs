use aframers::af_sys::entities::AEntity;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::aframe_ex::systems::{aframe_system_def, ASystem, register_system};

pub const NAME: &str = "hexcell";
const SELECTED_COLOR: &str = "thistle";
const PLAIN_COLOR: &str = "silver";

pub fn register() {
	let definition = aframe_system_def(HexcellSystem::new().into());
	register_system(NAME, &definition);
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = ASystem)]
	pub type HexcellSystemApi;
	#[wasm_bindgen(method)]
	pub fn ring_color(this: &HexcellSystemApi, cell: &AEntity) -> String;
}

#[wasm_bindgen]
pub struct HexcellSystem {
	selected: Option<AEntity>,
}

#[wasm_bindgen]
impl HexcellSystem {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self { selected: None }
	}
	pub fn __system_is_selected(&self, _system: &ASystem, cell: &AEntity) -> bool {
		match &self.selected {
			Some(value) => value.id() == cell.id(),
			None => false,
		}
	}
	pub fn __system_select_cell(&mut self, _system: &ASystem, cell: &AEntity) {
		self.selected = Some(cell.clone());
	}
	pub fn __system_selected(&self, _system: &ASystem) -> Option<AEntity> {
		self.selected.clone()
	}
	pub fn __system_ring_color(&self, system: &ASystem, cell: &AEntity) -> String {
		if self.__system_is_selected(system, cell) {
			SELECTED_COLOR.into()
		} else {
			PLAIN_COLOR.into()
		}
	}
}
