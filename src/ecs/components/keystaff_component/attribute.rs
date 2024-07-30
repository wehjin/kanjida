use aframers::components::core::ComponentAttribute;

use crate::ecs::components::keystaff_component::COMPONENT_NAME;

pub struct Keystaff;
impl ComponentAttribute for Keystaff {
	fn as_attribute_name(&self) -> impl AsRef<str> { COMPONENT_NAME }
	fn as_attribute_str(&self) -> impl AsRef<str> { "true" }
}
