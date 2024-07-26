use aframers::components::Color;
use aframers::components::core::{ComponentAttribute, ComponentSetting};

#[derive(Default)]
pub struct Material {
	color: Option<Color>,
	opacity: Option<f32>,
	transparent: Option<bool>,
	shader: Option<String>,
	target: Option<String>,
}

impl Material {
	pub fn new() -> Self { Self::default() }
	pub fn set_color(self, value: Color) -> Self {
		Self { color: Some(value), ..self }
	}
	pub fn set_opacity(self, value: f32) -> Self {
		Self { opacity: Some(value), ..self }
	}
	pub fn set_transparent(self, value: bool) -> Self {
		Self { transparent: Some(value), ..self }
	}
	pub fn set_shader(self, value: impl AsRef<str>) -> Self {
		Self { shader: Some(value.as_ref().to_string()), ..self }
	}
	pub fn set_target(self, value: impl AsRef<str>) -> Self {
		Self { target: Some(value.as_ref().to_string()), ..self }
	}
}

impl ComponentAttribute for Material {
	fn as_attribute_name(&self) -> impl AsRef<str> { "material" }

	fn as_attribute_str(&self) -> impl AsRef<str> {
		let mut clauses = vec![];
		if let Some(color) = &self.color {
			clauses.push(format!("color: {}", color.as_setting_str().as_ref()));
		}
		if let Some(value) = self.opacity {
			clauses.push(format!("opacity: {}", value));
		}
		if let Some(value) = self.transparent {
			clauses.push(format!("transparent: {}", value.to_string()));
		}
		if let Some(value) = &self.shader {
			clauses.push(format!("shader: {}", value));
		}
		if let Some(value) = &self.target {
			clauses.push(format!("target: {}", value));
		}
		clauses.join("; ")
	}
}