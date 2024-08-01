use aframers::af_sys::components::AComponent;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::aframe_ex::components::laser_controls_component::Hand;
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::ecs::systems::keystaff_system::ACTIVE_SHIELD;
use crate::ecs::systems::keystaff_system::shield_bank::ShieldBank;
use crate::ecs::systems::keystaff_system::shield_point::ShieldPoint;
use crate::three_sys::Vector3;

#[wasm_bindgen]
pub struct TickTask {
	pub(crate) vec3: Vector3,
	pub(crate) row2_min: f32,
	pub(crate) row2_max: f32,
	pub(crate) col2_min: f32,
	pub(crate) col2_max: f32,
	pub(crate) crown: AEntityEx,
	pub(crate) bank: ShieldBank,
}
impl TickTask {
	pub fn current_shield_point(&self) -> ShieldPoint {
		ACTIVE_SHIELD.with_borrow(|shield| shield.active_point())
	}
	pub fn set_current_shield_point(&mut self, value: ShieldPoint) {
		ACTIVE_SHIELD.with_borrow_mut(|shield| shield.set_active_point(value));
	}
	pub fn try_glyph(&self) -> Option<&'static str> {
		ACTIVE_SHIELD.with_borrow(|shield| shield.try_glyph())
	}
	pub fn has_active_bank(&self) -> bool {
		self.bank == ACTIVE_SHIELD.with_borrow(|shield| shield.active_bank())
	}
	pub fn update_bank_to_active(&mut self) {
		self.bank = ACTIVE_SHIELD.with_borrow(|shield| shield.active_bank());
	}
}

#[wasm_bindgen]
pub struct KeystaffState {
	pub(crate) hand: Hand,
	pub(crate) keystaff: AEntityEx,
	pub(crate) tick_task: Option<TickTask>,
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AComponent)]
	pub type KeystaffAComponent;
	#[wasm_bindgen(method, getter, js_name = keystaffState)]
	pub fn take_keystaff_state(this: &KeystaffAComponent) -> KeystaffState;
	#[wasm_bindgen(method, setter, js_name = keystaffState)]
	pub fn set_keystaff_state(this: &KeystaffAComponent, state: KeystaffState);
}
