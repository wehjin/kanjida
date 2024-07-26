use aframers::components::core::ComponentAttribute;

#[derive(Default)]
pub struct Renderer {
	antialias: Option<bool>,
}

impl Renderer {
	pub fn new() -> Self { Self::default() }
	pub fn set_antialias(self, value: bool) -> Self {
		Self { antialias: Some(value), ..self }
	}
}

impl ComponentAttribute for Renderer {
	fn as_attribute_name(&self) -> impl AsRef<str> { "renderer" }

	fn as_attribute_str(&self) -> impl AsRef<str> {
		let mut clauses = vec![];
		if let Some(value) = self.antialias {
			clauses.push(format!("antialias: {}", value))
		}
		clauses.join("; ")
	}
}