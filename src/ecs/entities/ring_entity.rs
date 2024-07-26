use aframers::components::Color;
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::material_component::Material;
use crate::aframe_ex::RingGeometry;

pub fn try_ring_entity(color: impl AsRef<str>) -> Result<Entity, JsValue> {
	let entity = create_entity()?
		.set_component_attribute(ring_material(color))?
		.set_component_attribute(ring_geometry())?
		;
	Ok(entity)
}

fn ring_geometry() -> RingGeometry {
	let geometry = RingGeometry::default()
		.set_segments_theta(6)
		.set_radius_outer(1.0)
		;
	geometry
}

fn ring_material(color: impl AsRef<str> + Sized) -> Material {
	let material = Material::new().set_color(Color::Web(color.as_ref().into()));
	material
}