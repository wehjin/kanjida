use aframers::components::core::ComponentAttribute;

pub struct Stats;
impl ComponentAttribute for Stats {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		"stats"
	}
	fn as_attribute_str(&self) -> impl AsRef<str> {
		"true"
	}
}