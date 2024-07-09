use aframers::component::Color;
use aframers::entity::{create_entity, Entity};
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::geometry_component::{Box, Geometry};
use crate::aframe_ex::components::material::Material;
use crate::components::collider_check_component::ColliderCheck;

pub fn make() -> Result<Entity, JsValue> {
	let chest = create_entity()?
		.set_component(Geometry::<Box>::new().set_primitive())?
		.set_component(Material::new().set_shader("html").set_target("#htmlElement"))?
		.set_component(Color::Web("goldenrod"))?
		.set_component(ColliderCheck)?
		;
	Ok(chest)
}