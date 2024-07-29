use aframers::components::core::{ComponentAttribute, ComponentSetting};

pub struct Value<T: AsRef<str> + Sized>(pub T);

impl<T: AsRef<str> + Sized> AsRef<str> for Value<T> {
	fn as_ref(&self) -> &str {
		self.0.as_ref()
	}
}

impl<T: AsRef<str> + Sized> ComponentSetting for Value<T> {
	fn as_setting_name(&self) -> impl AsRef<str> { "value" }
	fn as_setting_str(&self) -> impl AsRef<str> { self }
}

impl<T: AsRef<str> + Sized> ComponentAttribute for Value<T> {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		self.as_setting_name()
	}
	fn as_attribute_str(&self) -> impl AsRef<str> {
		self.as_setting_str()
	}
}