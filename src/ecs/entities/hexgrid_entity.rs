use aframers::af_sys::entities::a_entity_create;
use aframers::components::{Radius, ThetaStart};
use aframers::entities::Entity;
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::source_component::Source;
use crate::ecs::components::hexgrid_component::hexgrid::Hexgrid;

pub fn create_hexgrid() -> Result<Entity, JsValue> {
	let entity = circle_entity()?
		.set_component(Hexgrid::Enabled)?
		.set_component(Source::new("#spiral"))?
		;
	Ok(entity)
}

fn circle_entity() -> Result<Entity, JsValue> {
	let element = a_entity_create("a-circle")?;
	element.set_attribute("segments", "6")?;
	let entity = Entity::from(element)
		.set_component(Radius(71. / 2.))?
		.set_component(ThetaStart(30.))?
		;
	Ok(entity)
}
