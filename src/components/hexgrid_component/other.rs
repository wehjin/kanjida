use aframers::af_sys::entities::AEntity;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone)]
pub struct SelectedEntity {
	entity: Option<AEntity>,
}

#[wasm_bindgen]
impl SelectedEntity {
	#[wasm_bindgen(constructor)]
	pub fn none() -> Self {
		Self { entity: None }
	}
	pub fn replace_entity(&mut self, value: &AEntity) -> Option<AEntity> {
		match &self.entity {
			None => {
				self.entity = Some(value.clone());
				None
			}
			Some(old_value) if old_value.id() == value.id() => {
				None
			}
			Some(old_value) => {
				let old_value = old_value.clone();
				self.entity = Some(value.clone());
				Some(old_value)
			}
		}
	}
	pub fn remove_entity_if_present(&mut self, value: &AEntity) {
		if let Some(existing) = &self.entity {
			if existing.id() == value.id() {
				self.entity = None;
			}
		}
	}
}