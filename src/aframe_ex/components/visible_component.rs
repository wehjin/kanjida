use aframers::components::core::ComponentAttribute;

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
impl ComponentAttribute for Visible {
	fn as_attribute_name(&self) -> impl AsRef<str> { "visible" }
	fn as_attribute_str(&self) -> impl AsRef<str> { self }
}