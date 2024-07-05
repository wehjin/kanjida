use aframers::entity::{create_camera_entity, Entity};
use wasm_bindgen::JsValue;
use aframers::component::Position;

pub fn make() -> Result<Entity, JsValue> {
	let camera = create_camera_entity()?
		.set_component(Position(0.0, 1.6, 0.0))?
		;
	Ok(camera)
}