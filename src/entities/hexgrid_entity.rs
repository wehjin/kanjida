use aframers::components::{Height, Width};
use aframers::entities::{create_plane_entity, Entity};
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::source_component::Source;

pub fn try_make() -> Result<Entity, JsValue> {
	//create_entity()?.set_component(Hexgrid::Enabled)

	let entity = create_plane_entity()?
		.set_component(Width(71.0))?
		.set_component(Height(71.0))?
		.set_component(Source::new("#spiral"))?
		;

	Ok(entity)
}