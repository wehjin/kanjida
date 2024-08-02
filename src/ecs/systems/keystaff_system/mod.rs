use std::cell::RefCell;
use std::thread::LocalKey;

use wasm_bindgen::prelude::wasm_bindgen;

use shield_bank::ShieldBank;
use shield_point::ShieldPoint;

use crate::aframe_ex::components::laser_controls_component::Hand;

pub mod shield_point;
pub mod shield_bank;

thread_local! {
	pub static RIGHT_SHIELD: RefCell<Shield> = RefCell::new(Shield::new(Hand::Right));
	pub static LEFT_SHIELD: RefCell<Shield> = RefCell::new(Shield::new(Hand::Left));
}

pub fn hand_shield(hand: Hand) -> &'static LocalKey<RefCell<Shield>> {
	match hand {
		Hand::Right => &RIGHT_SHIELD,
		Hand::Left => &LEFT_SHIELD,
	}
}

#[wasm_bindgen]
pub fn select_bank(bank: &str) {
	let bank = ShieldBank::from(bank);
	RIGHT_SHIELD.with_borrow_mut(|shield| shield.set_active_bank(bank));
}

pub struct Shield {
	hand: Hand,
	active_bank: ShieldBank,
	active_point: ShieldPoint,
}

impl Shield {
	pub fn new(hand: Hand) -> Self {
		Self { active_bank: ShieldBank::N, active_point: ShieldPoint::CenterMiddle, hand }
	}
	pub fn try_glyph(&self) -> Option<&'static str> {
		self.active_bank.try_glyph(self.active_point, self.hand)
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

