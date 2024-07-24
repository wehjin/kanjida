use aframers::components::core::ComponentValue;
use crate::ecs::components::hexgrid_component::HEXGRID_COMPONENT_NAME;

pub enum Hexgrid { Enabled }

impl AsRef<str> for Hexgrid {
	fn as_ref(&self) -> &str {
		match self { Hexgrid::Enabled => "enabled" }
	}
}

impl ComponentValue for Hexgrid {
	fn component_name(&self) -> &str { HEXGRID_COMPONENT_NAME }
	fn component_value(&self) -> impl AsRef<str> { "enabled" }
}