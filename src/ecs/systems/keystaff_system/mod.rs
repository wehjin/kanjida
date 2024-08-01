use std::cell::RefCell;

use wasm_bindgen::prelude::wasm_bindgen;

use shield_bank::ShieldBank;
use shield_point::ShieldPoint;

pub mod shield_point;
pub mod shield_bank;

thread_local! {
	pub static ACTIVE_SHIELD: RefCell<Shield> = RefCell::new(Shield::new());
}

#[wasm_bindgen]
pub fn select_bank(bank: &str) {
	let bank = ShieldBank::from(bank);
	ACTIVE_SHIELD.with_borrow_mut(|shield| shield.set_active_bank(bank));
}

pub struct Shield {
	active_bank: ShieldBank,
	active_point: ShieldPoint,
}

impl Shield {
	pub fn new() -> Self {
		Self { active_bank: ShieldBank::N, active_point: ShieldPoint::CenterMiddle }
	}
	pub fn try_glyph(&self) -> Option<&'static str> {
		self.active_bank.try_glyph(self.active_point)
	}
	pub fn active_bank(&self) -> ShieldBank {
		self.active_bank
	}
	pub fn set_active_bank(&mut self, value: ShieldBank) {
		self.active_bank = value;
	}
	pub fn active_point(&self) -> ShieldPoint {
		self.active_point
	}
	pub fn set_active_point(&mut self, value: ShieldPoint) {
		self.active_point = value;
	}
}

