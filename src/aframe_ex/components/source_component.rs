use aframers::components::core::ComponentValue;

#[derive(Debug, Clone)]
pub struct Source(String);
impl Source {
	pub fn new(value: impl AsRef<str>) -> Self {
		Source(value.as_ref().to_string())
	}
}
impl AsRef<str> for Source {
	fn as_ref(&self) -> &str { &self.0 }
}
impl ComponentValue for Source {
	fn component_name(&self) -> &str { "src" }
	fn component_value(&self) -> impl AsRef<str> { self }
}