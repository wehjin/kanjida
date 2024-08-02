use aframers::af_sys::components::AComponent;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::aframe_ex::components::laser_controls_component::Hand;
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::ecs::systems::keystaff_system::hand_shield;
use crate::ecs::systems::keystaff_system::shield_bank::ShieldBank;
use crate::ecs::systems::keystaff_system::shield_point::ShieldPoint;
use crate::three_sys::Vector3;

#[wasm_bindgen]
pub struct TickTask {
	pub(crate) row2_min: f32,
	pub(crate) row2_max: f32,
	pub(crate) col2_min: f32,
	pub(crate) col2_max: f32,
	pub(crate) crown: AEntityEx,
	pub(crate) bank: ShieldBank,
	pub(crate) hand: Hand,
}
impl TickTask {
	pub fn current_shield_point(&self) -> ShieldPoint {
		hand_shield(self.hand).with_borrow(|shield| shield.active_point())
	}
	pub fn update_current_shield_point(&mut self, value: ShieldPoint) {
		hand_shield(self.hand).with_borrow_mut(|shield| shield.set_active_point(value));
	}
	pub fn try_glyph(&self) -> Option<&'static str> {
		hand_shield(self.hand).with_borrow(|shield| shield.try_glyph())
	}
	pub fn has_active_bank(&self) -> bool {
		self.bank == hand_shield(self.hand).with_borrow(|shield| shield.active_bank())
	}
	pub fn update_bank_to_active(&mut self) {
		self.bank = hand_shield(self.hand).with_borrow(|shield| shield.active_bank());
	}
}

#[wasm_bindgen]
pub struct KeystaffState {
	pub(crate) hand: Hand,
	pub(crate) keystaff: AEntityEx,
	pub(crate) tick_task: Option<TickTask>,
	pub(crate) vector3: Vector3,
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

impl KeystaffAComponent {
	pub fn to_controller(&self) -> AEntityEx {
		self.a_entity().unchecked_into::<AEntityEx>()
	}
}