use aframers::components::{Color, Height, Position, Rotation, Width};
use aframers::entities::{create_plane_entity, Entity};
use wasm_bindgen::JsValue;

pub fn make() -> Result<Entity, JsValue> {
	let size = 3.0;
	let entity = create_plane_entity()?
		.set_component_attribute(Position(0.0, 0.0, 0.0))?
		.set_component_attribute(Rotation(-90.0, 0.0, 0.0))?
		.set_component_attribute(Width(size))?
		.set_component_attribute(Height(size))?
		.set_component_attribute(Color::WebStr("#7BC8A4"))?;
	Ok(entity)
}