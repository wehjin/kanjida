use aframers::components::core::ComponentAttribute;

pub struct Game;

impl ComponentAttribute for Game {
	fn as_attribute_name(&self) -> impl AsRef<str> { "game" }
	fn as_attribute_str(&self) -> impl AsRef<str> { "true" }
}

