use std::fmt::{Display, Formatter};

use aframers::components::{Height, Width};
use aframers::components::core::{ComponentAttribute, ComponentSetting};

pub mod af_sys;
pub mod components;
pub mod entities;
pub mod events;
pub mod js;
pub mod objects;
pub mod scenes;
pub mod schema;
pub mod systems;

#[derive(Clone, Default)]
pub struct Text {
	align: Option<Align>,
	anchor: Option<Anchor>,
	baseline: Option<Baseline>,
	font: Option<String>,
	height: Option<Height>,
	value: Option<String>,
	width: Option<Width>,
	wrap_count: Option<f32>,
	z_offset: Option<f32>,
}

impl Text {
	pub fn set_align(mut self, value: Align) -> Self {
		self.align = Some(value);
		self
	}
	pub fn set_anchor(mut self, value: Anchor) -> Self {
		self.anchor = Some(value);
		self
	}
	pub fn set_baseline(mut self, value: Baseline) -> Self {
		self.baseline = Some(value);
		self
	}
	pub fn set_font(mut self, value: impl AsRef<str>) -> Self {
		self.font = Some(value.as_ref().to_string());
		self
	}
	pub fn set_height(mut self, value: Height) -> Self {
		self.height = Some(value);
		self
	}
	pub fn set_value(self, value: impl AsRef<str>) -> Self {
		Self { value: Some(value.as_ref().into()), ..self }
	}
	pub fn set_width(mut self, value: Width) -> Self {
		self.width = Some(value);
		self
	}
	pub fn set_wrap_count(mut self, value: f32) -> Self {
		self.wrap_count = Some(value);
		self
	}
	pub fn set_z_offset(self, value: f32) -> Self {
		Self { z_offset: Some(value), ..self }
	}
	pub fn new() -> Self {
		Self {
			align: None,
			anchor: None,
			baseline: None,
			font: None,
			height: None,
			value: None,
			width: None,
			wrap_count: None,
			z_offset: None,
		}
	}
}

impl ComponentAttribute for Text {
	fn as_attribute_name(&self) -> impl AsRef<str> { "text" }

	fn as_attribute_str(&self) -> impl AsRef<str> {
		let mut clauses = vec![];
		if let Some(value) = self.align {
			clauses.push(format!("align: {}", value));
		}
		if let Some(value) = self.anchor {
			clauses.push(format!("anchor: {}", value));
		}
		if let Some(value) = self.baseline {
			clauses.push(format!("baseline: {}", value));
		}
		if let Some(value) = &self.font {
			clauses.push("negate: false".into());
			clauses.push(format!("font: {}", value));
		}
		if let Some(value) = self.height {
			clauses.push(format!("height: {}", value.as_setting_str().as_ref()));
		}
		if let Some(value) = &self.value {
			clauses.push(format!("value: {}", value));
		}
		if let Some(value) = self.width {
			clauses.push(format!("width: {}", value.as_setting_str().as_ref()));
		}
		if let Some(value) = self.wrap_count {
			clauses.push(format!("wrapCount: {}", value));
		}
		if let Some(value) = self.z_offset {
			clauses.push(format!("zOffset: {}", value));
		}
		clauses.join("; ")
	}
}

#[derive(Copy, Clone)]
pub enum Anchor { Left, Center, Right, Align }
impl AsRef<str> for Anchor {
	fn as_ref(&self) -> &str {
		match self {
			Anchor::Left => "left",
			Anchor::Center => "center",
			Anchor::Right => "right",
			Anchor::Align => "align",
		}
	}
}
impl ComponentSetting for Anchor {
	fn as_setting_name(&self) -> impl AsRef<str> { "anchor" }
	fn as_setting_str(&self) -> impl AsRef<str> { self }
}
impl ComponentAttribute for Anchor {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		self.as_setting_name()
	}

	fn as_attribute_str(&self) -> impl AsRef<str> {
		self.as_setting_str()
	}
}
impl Display for Anchor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}

pub struct Value<T: AsRef<str> + Sized>(pub T);
impl<T: AsRef<str> + Sized> AsRef<str> for Value<T> {
	fn as_ref(&self) -> &str {
		self.0.as_ref()
	}
}
impl<T: AsRef<str> + Sized> ComponentSetting for Value<T> {
	fn as_setting_name(&self) -> impl AsRef<str> { "value" }
	fn as_setting_str(&self) -> impl AsRef<str> { self }
}
impl<T: AsRef<str> + Sized> ComponentAttribute for Value<T> {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		self.as_setting_name()
	}
	fn as_attribute_str(&self) -> impl AsRef<str> {
		self.as_setting_str()
	}
}

#[derive(Copy, Clone)]
pub enum Align { Left, Center, Right }
impl AsRef<str> for Align {
	fn as_ref(&self) -> &str {
		match self {
			Align::Left => "left",
			Align::Center => "center",
			Align::Right => "right",
		}
	}
}
impl ComponentSetting for Align {
	fn as_setting_name(&self) -> impl AsRef<str> { "align" }
	fn as_setting_str(&self) -> impl AsRef<str> { self }
}
impl ComponentAttribute for Align {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		self.as_setting_name()
	}

	fn as_attribute_str(&self) -> impl AsRef<str> {
		self.as_setting_str()
	}
}
impl Display for Align {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}

#[derive(Copy, Clone)]
pub enum Baseline { Top, Center, Bottom }
impl AsRef<str> for Baseline {
	fn as_ref(&self) -> &str {
		match self {
			Baseline::Top => "top",
			Baseline::Center => "center",
			Baseline::Bottom => "bottom",
		}
	}
}
impl ComponentSetting for Baseline {
	fn as_setting_name(&self) -> impl AsRef<str> { "baseline" }
	fn as_setting_str(&self) -> impl AsRef<str> { self }
}
impl ComponentAttribute for Baseline {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		self.as_setting_name()
	}

	fn as_attribute_str(&self) -> impl AsRef<str> {
		self.as_setting_str()
	}
}
impl Display for Baseline {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}

#[derive(Copy, Clone, Default)]
pub struct RingGeometry {
	radius_inner: Option<f32>,
	radius_outer: Option<f32>,
	segments_theta: Option<u32>,
	segments_phi: Option<u32>,
	theta_start: Option<f32>,
	theta_length: Option<f32>,
}

impl RingGeometry {
	pub fn set_segments_theta(mut self, value: u32) -> Self {
		self.segments_theta = Some(value);
		self
	}
	pub fn set_radius_outer(mut self, value: f32) -> Self {
		self.radius_outer = Some(value);
		self
	}
}

impl ComponentAttribute for RingGeometry {
	fn as_attribute_name(&self) -> impl AsRef<str> { "geometry" }

	fn as_attribute_str(&self) -> impl AsRef<str> {
		let mut clauses = vec!["primitive:ring".to_string()];
		if let Some(value) = self.radius_inner {
			clauses.push(format!("radiusInner: {}", value));
		}
		if let Some(value) = self.radius_outer {
			clauses.push(format!("radiusOuter: {}", value));
		}
		if let Some(value) = self.segments_theta {
			clauses.push(format!("segmentsTheta: {}", value));
		}
		if let Some(value) = self.segments_phi {
			clauses.push(format!("segmentsPhi: {}", value));
		}
		if let Some(value) = self.theta_start {
			clauses.push(format!("thetaStart: {}", value));
		}
		if let Some(value) = self.theta_length {
			clauses.push(format!("thetaLength: {}", value));
		}
		let value = clauses.join("; ");
		value
	}
}

