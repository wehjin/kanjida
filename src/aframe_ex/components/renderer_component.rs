use aframers::component::core::ComponentValue;

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

impl ComponentValue for Renderer {
	fn component_name(&self) -> &str { "renderer" }

	fn component_value(&self) -> impl AsRef<str> {
		let mut clauses = vec![];
		if let Some(value) = self.antialias {
			clauses.push(format!("antialias: {}", value))
		}
		clauses.join("; ")
	}
}