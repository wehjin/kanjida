use aframers::components::core::ComponentValue;

use crate::aframe_ex::schema::properties::Field;

#[derive(Debug, Clone)]
pub enum YomigunSetting {
	YomiCode(usize)
}
impl YomigunSetting {
	pub fn to_field(&self) -> Field {
		match self {
			YomigunSetting::YomiCode(code) => Field::usize(*code)
		}
	}
}
impl ComponentValue for YomigunSetting {
	fn component_name(&self) -> &str {
		match self {
			YomigunSetting::YomiCode(_) => "yomiCode"
		}
	}
	fn component_value(&self) -> impl AsRef<str> {
		match self {
			YomigunSetting::YomiCode(value) => format!("{}", *value)
		}
	}
}
