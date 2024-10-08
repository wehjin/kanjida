use aframers::components::core::ComponentAttribute;

#[derive(Default)]
pub struct Hexcell {
	glyph: Option<String>,
	solved: Option<bool>,
}

impl Hexcell {
	pub fn new() -> Self { Self::default() }
	pub fn set_glyph(self, value: impl AsRef<str>) -> Self {
		Self { glyph: Some(value.as_ref().to_string()), ..self }
	}
	pub fn set_solved(self, value: bool) -> Self {
		Self { solved: Some(value), ..self }
	}
}
impl ComponentAttribute for Hexcell {
	fn as_attribute_name(&self) -> impl AsRef<str> { "hexcell" }

	fn as_attribute_str(&self) -> impl AsRef<str> {
		let mut clauses = vec![];
		if let Some(value) = &self.glyph {
			clauses.push(format!("glyph: {}", value));
		}
		if let Some(value) = self.solved {
			clauses.push(format!("solved: {}", value.to_string()))
		}
		clauses.join("; ")
	}
}
