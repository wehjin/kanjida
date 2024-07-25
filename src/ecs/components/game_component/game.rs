use aframers::components::core::ComponentValue;

pub struct Game;

impl ComponentValue for Game {
	fn component_name(&self) -> &str { "game" }
	fn component_value(&self) -> impl AsRef<str> { "true" }
}

