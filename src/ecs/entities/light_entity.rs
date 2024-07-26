use aframers::components::{Color, Position};
use aframers::entities::{create_light_entity, Entity};
use wasm_bindgen::JsValue;

pub fn make_over() -> Result<Entity, JsValue> {
	let entity = create_light_entity()?
		.set_component_attribute(Color::Web("#fff".into()))?
		.set_component_attribute(Position(-1.0, 2.0, 4.))?;
	Ok(entity)
}
