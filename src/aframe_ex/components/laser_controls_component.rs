use std::cell::LazyCell;

use aframers::components::core::{ComponentAttribute, ComponentSetting};
use wasm_bindgen::JsValue;

use crate::aframe_ex::schema::{SchemaProperty, SchemaPropertyType};
use crate::aframe_ex::schema::properties::StringSchemaProperty;
use crate::js_sys_ex::JsKey;

thread_local! {
	pub static HAND_KEY: LazyCell<JsKey> = LazyCell::new(||"hand".into())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum Hand {
	#[default]
	Right,
	Left,
}
impl Hand {
	pub fn as_str(&self) -> &str {
		match self {
			Hand::Left => "left",
			Hand::Right => "right",
		}
	}
}
impl<T: AsRef<str>> From<T> for Hand {
	fn from(value: T) -> Self {
		let value = value.as_ref().trim();
		match value {
			"left" => Self::Left,
			"right" => Self::Right,
			_ => panic!("Invalid source value '{}' for Hand", value)
		}
	}
}
impl ComponentSetting for Hand {
	fn as_setting_name(&self) -> impl AsRef<str> {
		"hand"
	}

	fn as_setting_str(&self) -> impl AsRef<str> {
		self.as_str()
	}
}
impl SchemaProperty for Hand {
	fn to_schema_property_type(&self) -> SchemaPropertyType {
		SchemaPropertyType::String
	}
	fn format(&self) -> String {
		StringSchemaProperty::format(self.as_str())
	}
	fn create_js(&self) -> JsValue {
		StringSchemaProperty::create_js(self.as_str())
	}
	fn parse_js(data: &JsValue) -> Self {
		let left_or_right = StringSchemaProperty::parse_js(data);
		Self::from(left_or_right)
	}
}

pub struct LaserControls {
	pub hand: Option<Hand>,
}

impl LaserControls {
	pub fn new() -> Self { Self { hand: None } }
	pub fn set_hand(mut self, hand: Hand) -> Self {
		self.hand = Some(hand);
		self
	}
}

impl ComponentAttribute for LaserControls {
	fn as_attribute_name(&self) -> impl AsRef<str> { "laser-controls" }

	fn as_attribute_str(&self) -> impl AsRef<str> {
		let mut clauses = vec![];
		if let Some(hand) = &self.hand {
			let clause = format!("{}: {}", hand.as_setting_name().as_ref(), hand.as_setting_str().as_ref());
			clauses.push(clause);
		}
		clauses.join("; ")
	}
}
