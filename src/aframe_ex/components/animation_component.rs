use aframers::components::core::{ComponentAttribute, ToPropertyValue};

use crate::aframe_ex::components::core::properties::{AsPropertyName, ComponentProperty, MultiPropertyAttributeValue};

pub const ANIMATION: &'static str = "animation";


#[derive(Debug, Copy, Clone)]
pub enum AnimationEvent {
	AnimationBegin,
	AnimationComplete,
}
impl AsRef<str> for AnimationEvent {
	//noinspection SpellCheckingInspection
	fn as_ref(&self) -> &str {
		match self {
			AnimationEvent::AnimationBegin => "animationbegin",
			AnimationEvent::AnimationComplete => "animationcomplete"
		}
	}
}

pub enum Easing {
	EaseInQuad,
	EaseOutQuad,
	EaseInOutQuad,
	Linear,
}
impl AsRef<str> for Easing {
	fn as_ref(&self) -> &str {
		match self {
			Easing::Linear => "linear",
			Easing::EaseInQuad => "easeInQuad",
			Easing::EaseOutQuad => "easeOutQuad",
			Easing::EaseInOutQuad => "easeInOutQuad",
		}
	}
}

impl AsPropertyName for Easing {
	fn as_property_name(&self) -> &str {
		"easing"
	}
}

impl ToPropertyValue for Easing {
	fn to_property_value(&self) -> String { format!("{}", self.as_ref()) }
}
impl ComponentProperty for Easing {}

#[derive(Default)]
pub struct Animation {
	property_path: Option<String>,
	is_raw_property_path: Option<bool>,
	from: Option<Box<dyn ToPropertyValue>>,
	to: Option<Box<dyn ToPropertyValue>>,
	delay_millis: Option<u32>,
	dur_millis: Option<u32>,
	easing: Option<Easing>,
}

impl Animation {
	pub fn new() -> Self {
		Self {
			property_path: None,
			is_raw_property_path: None,
			from: None,
			to: None,
			delay_millis: None,
			dur_millis: None,
			easing: None,
		}
	}
	pub fn set_property(self, value: impl AsRef<str>) -> Self {
		Self { property_path: Some(value.as_ref().into()), ..self }
	}
	pub fn set_is_raw_property(self, value: bool) -> Self {
		Self { is_raw_property_path: Some(value), ..self }
	}
	pub fn set_from(self, value: impl ToPropertyValue + 'static) -> Self {
		Self { from: Some(Box::new(value)), ..self }
	}
	pub fn set_to(self, value: impl ToPropertyValue + 'static) -> Self {
		Self { to: Some(Box::new(value)), ..self }
	}
	pub fn set_delay_millis(self, value: u32) -> Self {
		Self { delay_millis: Some(value), ..self }
	}
	pub fn set_dur_millis(self, value: u32) -> Self {
		Self { dur_millis: Some(value), ..self }
	}
	pub fn set_easing(self, value: Easing) -> Self {
		Self { easing: Some(value), ..self }
	}
}


impl ComponentAttribute for Animation {
	fn as_attribute_name(&self) -> impl AsRef<str> { ANIMATION }

	fn as_attribute_str(&self) -> impl AsRef<str> {
		MultiPropertyAttributeValue::new()
			.add_property_value("property", &self.property_path)
			.add_property_value("isRawProperty", &self.is_raw_property_path)
			.add_property_value("from", &self.from)
			.add_property_value("to", &self.to)
			.add_property_value("delay", &self.delay_millis)
			.add_property_value("dur", &self.dur_millis)
			.add_property(&self.easing)
			.to_attribute_value()
	}
}
