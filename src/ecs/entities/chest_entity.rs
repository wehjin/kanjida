use aframers::components::Color;
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::geometry_component::{Box, Geometry};
use crate::aframe_ex::components::material::Material;
use crate::ecs::components::yomigun_component::attribute::Yomigun;

pub fn make_chest_entity() -> Result<Entity, JsValue> {
	let chest = create_entity()?
		.set_id("yomigun")?
		.set_component(Geometry::<Box>::new().set_primitive())?
		.set_component(Material::new().set_shader("html").set_target("#htmlElement"))?
		.set_component(Color::Web("silver".into()))?
		.set_component(Yomigun::Enabled)?
		;
	Ok(chest)
}