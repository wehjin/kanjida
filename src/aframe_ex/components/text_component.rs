use aframers::components::{Color, Height, Width};
use aframers::components::core::{ComponentAttribute, ComponentSetting};
use crate::aframe_ex::components::baseline_component::Baseline;
use crate::aframe_ex::components::align_component::Align;
use crate::aframe_ex::components::anchor_component::Anchor;

#[derive(Clone, Default)]
pub struct Text {
	align: Option<Align>,
	anchor: Option<Anchor>,
	baseline: Option<Baseline>,
	color: Option<Color>,
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
	pub fn set_color(mut self, value: Color) -> Self {
		self.color = Some(value);
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
			color: None,
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
		if let Some(value) = &self.color {
			clauses.push(format!("color: {}", value.as_setting_str().as_ref()));
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