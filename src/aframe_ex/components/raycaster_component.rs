use aframers::components::core::{ComponentAttribute, ComponentSetting};

use crate::aframe_ex::components::core::component_settings_as_string;
use crate::aframe_ex::schema::properties::Vec3SchemaProperty;

pub const COMPONENT_NAME: &'static str = "raycaster";

pub enum RaycasterEvent {
	Intersected,
	IntersectedCleared,
}
impl RaycasterEvent {
	pub fn as_str(&self) -> &str {
		match self {
			RaycasterEvent::Intersected => "raycaster-intersected",
			RaycasterEvent::IntersectedCleared => "raycaster-intersected-cleared",
		}
	}
}

#[derive(Debug)]
pub enum RaycasterSetting {
	Direction(f32, f32, f32),
	Enabled(bool),
	Objects(String),
	ShowLine(bool),
	UseWorldCoordinates(bool),
}

impl ComponentSetting for RaycasterSetting {
	fn as_setting_name(&self) -> impl AsRef<str> {
		match self {
			Self::Direction(_, _, _) => "direction",
			Self::Enabled(_) => "enabled",
			Self::Objects(_) => "objects",
			Self::ShowLine(_) => "showLine",
			Self::UseWorldCoordinates(_) => "useWorldCoordinates",
		}
	}
	fn as_setting_str(&self) -> impl AsRef<str> {
		match self {
			Self::Direction(x, y, z) => Vec3SchemaProperty::format_float(*x, *y, *z),
			Self::Enabled(value) => format!("{}", value),
			Self::Objects(value) => value.to_string(),
			Self::ShowLine(value) => format!("{}", value),
			Self::UseWorldCoordinates(value) => format!("{}", value),
		}
	}
}

#[derive(Debug)]
pub struct Raycaster(pub Vec<RaycasterSetting>);
impl Raycaster {
	pub fn objects(value: impl AsRef<str>) -> Self {
		Self(vec![RaycasterSetting::Objects(value.as_ref().to_string())])
	}
	pub fn enabled(value: bool) -> Self {
		Self(vec![RaycasterSetting::Enabled(value)])
	}
	pub fn add_setting(mut self, setting: RaycasterSetting) -> Self {
		self.0.push(setting);
		self
	}
}

impl ComponentAttribute for Raycaster {
	fn as_attribute_name(&self) -> impl AsRef<str> { COMPONENT_NAME }

	fn as_attribute_str(&self) -> impl AsRef<str> {
		component_settings_as_string(&self.0)
	}
}

