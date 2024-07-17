use aframers::components::Color;
use aframers::components::core::ComponentValue;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HexColor {
	Selected,
	Focused,
	FocusedAndSelected,
	NeitherFocusedNorSelected,
}

impl HexColor {
	pub fn to_color(&self) -> Color {
		Color::Web(self.as_ref().to_string())
	}
}
impl AsRef<str> for HexColor {
	fn as_ref(&self) -> &str {
		match self {
			HexColor::Selected => "DeepSkyBlue",
			HexColor::Focused => "Gold",
			HexColor::FocusedAndSelected => "LightSkyBlue",
			HexColor::NeitherFocusedNorSelected => "Silver",
		}
	}
}

impl ComponentValue for HexColor {
	fn component_name(&self) -> &str { "color" }
	fn component_value(&self) -> impl AsRef<str> { self }
}
