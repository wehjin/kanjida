use aframers::components::core::{ComponentAttribute, ComponentSetting};
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::core::component_settings_as_string;
use crate::aframe_ex::components::laser_controls_component::{Hand, HAND_KEY};
use crate::aframe_ex::schema::{Schema, SchemaProperty};
use crate::aframe_ex::schema::fields::Field;
use crate::aframe_ex::schema::multi_property::MultiPropertySchema;
use crate::ecs::components::keystaff_component::COMPONENT_NAME;

pub enum KeystaffSetting {
	Hand(Hand)
}

impl ComponentSetting for KeystaffSetting {
	fn as_setting_name(&self) -> impl AsRef<str> {
		match self {
			KeystaffSetting::Hand(hand) => hand.as_setting_name(),
		}
	}
	fn as_setting_str(&self) -> impl AsRef<str> {
		match self {
			KeystaffSetting::Hand(hand) => hand.as_setting_str()
		}
	}
}

pub struct Keystaff(Vec<KeystaffSetting>);

impl Keystaff {
	pub fn schema() -> impl Schema {
		let hand = Hand::Right;
		MultiPropertySchema::new()
			.push(hand.as_setting_name(), Field::string(hand.as_setting_str()))
	}
	pub fn get_hand(data: &JsValue) -> Hand {
		let hand = HAND_KEY.with(|key| {
			let hand_data = key.to_value(data);
			Hand::parse_js(&hand_data)
		});
		hand
	}
}


impl ComponentAttribute for Keystaff {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		COMPONENT_NAME
	}
	fn as_attribute_str(&self) -> impl AsRef<str> {
		component_settings_as_string(&self.0)
	}
}
impl From<Hand> for Keystaff {
	fn from(value: Hand) -> Self {
		Self(vec![KeystaffSetting::Hand(value)])
	}
}
