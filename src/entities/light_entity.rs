use aframers::components::{Color, Position};
use aframers::entities::{create_light_entity, Entity};
use wasm_bindgen::JsValue;

pub fn make_over() -> Result<Entity, JsValue> {
	let entity = create_light_entity()?
		.set_component(Color::Web("#fff"))?
		.set_component(Position(0.0, 5.0, -0.5))?;
	Ok(entity)
}

pub fn make_under() -> Result<Entity, JsValue> {
	let entity = create_light_entity()?
		.set_component(Color::Web("#aaa"))?
		.set_component(Position(0., -0.25, 0.5))?
		;
	Ok(entity)
}