use aframers::components::Position;
use aframers::entities::{create_camera_entity, Entity};
use wasm_bindgen::JsValue;

pub fn make() -> Result<Entity, JsValue> {
	let camera = create_camera_entity()?
		.set_component_attribute(Position(0.0, 1.6, 0.0))?
		;
	Ok(camera)
}