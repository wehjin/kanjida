use aframers::components::Color;
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::geometry_component::{Box, Geometry};
use crate::aframe_ex::components::material::Material;

pub fn make() -> Result<Entity, JsValue> {
	let chest = create_entity()?
		.set_component(Geometry::<Box>::new().set_primitive())?
		.set_component(Material::new().set_shader("html").set_target("#htmlElement"))?
		.set_component(Color::Web("goldenrod".into()))?
		;
	Ok(chest)
}