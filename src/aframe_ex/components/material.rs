use aframers::component::Color;
use aframers::component::core::ComponentValue;

#[derive(Default)]
pub struct Material {
	color: Option<Color>,
}

impl Material {
	pub fn new() -> Self { Self::default() }
	pub fn set_color(self, color: Color) -> Self {
		Self { color: Some(color), ..self }
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
		clauses.join("; ")
	}
}