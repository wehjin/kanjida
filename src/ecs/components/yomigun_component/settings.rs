use aframers::components::core::ComponentSetting;

use crate::aframe_ex::schema::fields::Field;

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
impl ComponentSetting for YomigunSetting {
	fn as_setting_name(&self) -> impl AsRef<str> {
		match self {
			YomigunSetting::YomiCode(_) => "yomiCode"
		}
	}
	fn as_setting_str(&self) -> impl AsRef<str> {
		match self {
			YomigunSetting::YomiCode(value) => format!("{}", *value)
		}
	}
}
