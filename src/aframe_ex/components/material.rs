use aframers::component::Color;
use aframers::component::core::ComponentValue;

#[derive(Default)]
pub struct Material {
	color: Option<Color>,
	opacity: Option<f32>,
	transparent: Option<bool>,
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
}

impl ComponentValue for Material {
	fn component_name(&self) -> &str { "material" }

	fn component_value(&self) -> impl AsRef<str> {
		let mut clauses = vec![];
		if let Some(color) = &self.color {
			let clause = format!("color: {}", color.component_value().as_ref());
			clauses.push(clause);
		}
		if let Some(value) = self.opacity {
			let clause = format!("opacity: {}", value);
			clauses.push(clause);
		}
		if let Some(value) = self.transparent {
			let clause = format!("transparent: {}", value.to_string());
			clauses.push(clause);
		}
		clauses.join("; ")
	}
}