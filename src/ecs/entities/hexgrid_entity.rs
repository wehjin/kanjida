use aframers::af_sys::entities::a_entity_create;
use aframers::components::{Radius, ThetaStart};
use aframers::components::Color::Web;
use aframers::entities::Entity;
use wasm_bindgen::JsValue;

use crate::ecs::components::hexgrid_component::hexgrid::Hexgrid;

pub fn create_hexgrid() -> Result<Entity, JsValue> {
	let entity = circle_entity()?
		.set_component_attribute(Hexgrid::Enabled)?
		.set_component_attribute(Web("#345".into()))?
		;
	Ok(entity)
}

fn circle_entity() -> Result<Entity, JsValue> {
	let element = a_entity_create("a-circle")?;
	element.set_attribute("segments", "6")?;
	let entity = Entity::from(element)
		.set_component_attribute(Radius(72. / 2.))?
		.set_component_attribute(ThetaStart(30.))?
		;
	Ok(entity)
}
