use aframers::components::core::{ComponentAttribute, ComponentSetting};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub enum Baseline { Top, Center, Bottom }

impl AsRef<str> for Baseline {
	fn as_ref(&self) -> &str {
		match self {
			Baseline::Top => "top",
			Baseline::Center => "center",
			Baseline::Bottom => "bottom",
		}
	}
}

impl ComponentSetting for Baseline {
	fn as_setting_name(&self) -> impl AsRef<str> { "baseline" }
	fn as_setting_str(&self) -> impl AsRef<str> { self }
}

impl ComponentAttribute for Baseline {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		self.as_setting_name()
	}

	fn as_attribute_str(&self) -> impl AsRef<str> {
		self.as_setting_str()
	}
}

impl Display for Baseline {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}