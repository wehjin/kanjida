use aframers::af_sys::entities::AEntity;
use aframers::af_sys::systems::{ASystem, register_system};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::aframe_ex::systems::aframe_system_def;

pub const NAME: &str = "hexcell";
const SELECTED_COLOR: &str = "#003262";
const PLAIN_COLOR: &str = "silver";

pub fn register() {
	let definition = aframe_system_def(HexcellSystem::new().into());
	register_system(NAME, &definition);
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = ASystem)]
	pub type HexcellASystem;
	#[wasm_bindgen(method)]
	pub fn ring_color(this: &HexcellASystem, cell: &AEntity) -> String;
	#[wasm_bindgen(method)]
	pub fn select_cell(this: &HexcellASystem, cell: &AEntity) -> Option<AEntity>;
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
	pub fn __api_is_selected(&self, _system: &ASystem, cell: &AEntity) -> bool {
		match &self.selected {
			Some(value) => value.id() == cell.id(),
			None => false,
		}
	}
	pub fn __api_select_cell(&mut self, _system: &ASystem, new: &AEntity) -> Option<AEntity> {
		match self.selected.take() {
			None => {
				self.selected = Some(new.clone());
				return None;
			}
			Some(old) => {
				if new.is_equal_node(Some(&old)) {
					self.selected = Some(old);
					return None;
				} else {
					self.selected = Some(new.clone());
					return Some(old);
				}
			}
		}
	}
	pub fn __api_selected(&self, _system: &ASystem) -> Option<AEntity> {
		self.selected.clone()
	}
	pub fn __api_ring_color(&self, system: &ASystem, cell: &AEntity) -> String {
		if self.__api_is_selected(system, cell) {
			SELECTED_COLOR.into()
		} else {
			PLAIN_COLOR.into()
		}
	}
}
