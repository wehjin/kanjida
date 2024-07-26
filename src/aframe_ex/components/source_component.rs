use aframers::components::core::ComponentAttribute;

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
impl ComponentAttribute for Source {
	fn as_attribute_name(&self) -> impl AsRef<str> { "src" }
	fn as_attribute_str(&self) -> impl AsRef<str> { self }
}