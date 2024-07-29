use std::fmt::{Display, Formatter};
use aframers::components::core::{ComponentAttribute, ComponentSetting};

#[derive(Copy, Clone)]
pub enum Anchor { Left, Center, Right, Align }

impl AsRef<str> for Anchor {
	fn as_ref(&self) -> &str {
		match self {
			Anchor::Left => "left",
			Anchor::Center => "center",
			Anchor::Right => "right",
			Anchor::Align => "align",
		}
	}
}

impl ComponentSetting for Anchor {
	fn as_setting_name(&self) -> impl AsRef<str> { "anchor" }
	fn as_setting_str(&self) -> impl AsRef<str> { self }
}

impl ComponentAttribute for Anchor {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		self.as_setting_name()
	}

	fn as_attribute_str(&self) -> impl AsRef<str> {
		self.as_setting_str()
	}
}

impl Display for Anchor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}