use aframers::components::core::ComponentValue;

#[derive(Debug, Copy, Clone)]
pub enum Visible {
	True,
	False,
}
impl AsRef<str> for Visible {
	fn as_ref(&self) -> &str {
		match self {
			Visible::True => "true",
			Visible::False => "false",
		}
	}
}
impl ComponentValue for Visible {
	fn component_name(&self) -> &str { "visible" }
	fn component_value(&self) -> impl AsRef<str> { self }
}