use aframers::components::Color;
use aframers::entities::{create_sky_entity, Entity};
use wasm_bindgen::JsValue;

pub fn make() -> Result<Entity, JsValue> {
	let sky = create_sky_entity()?.set_component(Color::Web("#5C5C5C".into()))?;
	Ok(sky)
}