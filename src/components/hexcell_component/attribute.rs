use aframers::components::core::ComponentValue;
use aframers::components::Color;

#[derive(Default)]
pub struct Hexcell {
	glyph: Option<String>,
	ring_color: Option<Color>,
}

impl Hexcell {
	pub fn new() -> Self { Self::default() }
	pub fn set_glyph(self, value: impl AsRef<str>) -> Self {
		Self { glyph: Some(value.as_ref().to_string()), ..self }
	}
	pub fn set_ring_color(self, value: Color) -> Self {
		Self { ring_color: Some(value), ..self }
	}
}

impl ComponentValue for Hexcell {
	fn component_name(&self) -> &str { "hexcell" }

	fn component_value(&self) -> impl AsRef<str> {
		let mut clauses = vec![];
		if let Some(value) = &self.glyph {
			clauses.push(format!("glyph: {}", value));
		}
		if let Some(value) = &self.ring_color {
			clauses.push(format!("ring_color: {}", value.component_value().as_ref()))
		}
		clauses.join("; ")
	}
}