use aframers::af_sys::entities::a_entity_create;
use aframers::components::{Height, Radius, ThetaStart, Width};
use aframers::entities::{create_plane_entity, Entity};
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::source_component::Source;

pub fn try_make() -> Result<Entity, JsValue> {
	//create_entity()?.set_component(Hexgrid::Enabled)

	let entity = circle_entity()?
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

fn plane_entity() -> Result<Entity, JsValue> {
	let entity = create_plane_entity()?
		.set_component(Width(71.0))?
		.set_component(Height(71.0))?
		;
	Ok(entity)
}