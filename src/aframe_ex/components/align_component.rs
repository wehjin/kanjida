use aframers::components::core::{ComponentAttribute, ComponentSetting};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub enum Align { Left, Center, Right }

impl AsRef<str> for Align {
	fn as_ref(&self) -> &str {
		match self {
			Align::Left => "left",
			Align::Center => "center",
			Align::Right => "right",
		}
	}
}

impl ComponentSetting for Align {
	fn as_setting_name(&self) -> impl AsRef<str> { "align" }
	fn as_setting_str(&self) -> impl AsRef<str> { self }
}

impl ComponentAttribute for Align {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		self.as_setting_name()
	}

	fn as_attribute_str(&self) -> impl AsRef<str> {
		self.as_setting_str()
	}
}

impl Display for Align {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}