use aframers::components::core::ComponentValue;

#[derive(Default)]
pub struct Hexcell {
	glyph: Option<String>,
}

impl Hexcell {
	pub fn new() -> Self { Self::default() }
	pub fn set_glyph(self, value: impl AsRef<str>) -> Self {
		Self { glyph: Some(value.as_ref().to_string()), ..self }
	}
}

impl ComponentValue for Hexcell {
	fn component_name(&self) -> &str { "hexcell" }

	fn component_value(&self) -> impl AsRef<str> {
		let mut clauses = vec![];
		if let Some(value) = &self.glyph {
			clauses.push(format!("glyph: {}", value));
		}
		clauses.join("; ")
	}
}