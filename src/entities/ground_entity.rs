use aframers::entity::{create_plane_entity, Entity};
use wasm_bindgen::JsValue;
use aframers::component::{Color, Height, Position, Rotation, Width};

pub fn make() -> Result<Entity, JsValue> {
	let size = 5.0;
	let entity = create_plane_entity()?
		.set_component(Position(0.0, 0.0, 0.0))?
		.set_component(Rotation(-90.0, 0.0, 0.0))?
		.set_component(Width(size))?
		.set_component(Height(size))?
		.set_component(Color::Web("#7BC8A4"))?;
	Ok(entity)
}