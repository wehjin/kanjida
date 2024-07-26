use aframers::components::core::ComponentAttribute;

use crate::ecs::components::hexgrid_component::HEXGRID_COMPONENT_NAME;

pub enum Hexgrid { Enabled }

impl AsRef<str> for Hexgrid {
	fn as_ref(&self) -> &str {
		match self { Hexgrid::Enabled => "enabled" }
	}
}

impl ComponentAttribute for Hexgrid {
	fn as_attribute_name(&self) -> impl AsRef<str>{ HEXGRID_COMPONENT_NAME }
	fn as_attribute_str(&self) -> impl AsRef<str> { "enabled" }
}