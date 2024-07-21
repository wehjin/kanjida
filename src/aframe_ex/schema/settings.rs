pub trait ComponentSetting {
	fn as_setting_name(&self) -> impl AsRef<str>;
	fn as_setting_str(&self) -> impl AsRef<str>;
}

pub trait ComponentAttribute {
	fn as_attribute_name(&self) -> impl AsRef<str>;
	fn as_attribute_str(&self) -> impl AsRef<str>;
}
