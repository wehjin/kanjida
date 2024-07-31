use aframers::browser::document;
use aframers::components::{Color, Position};
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::{JsCast, JsValue};

use crate::aframe_ex::components::material_component::Material;
use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::geometries::cylinder_geometry::{CylinderGeometry, CylinderGeometrySetting};
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;

const ROD_HEIGHT: f32 = 0.4;
const ROD_RADIUS: f32 = 0.020;
const HAND_HEIGHT: f32 = 0.09;

pub const ENTITY_ID: &'static str = "keystaff";

pub fn create_keystaff() -> Result<Entity, JsValue> {
	let material = Material::new().set_color(get_default_color());
	let geometry_settings = vec![
		CylinderGeometrySetting::Primitive,
		CylinderGeometrySetting::Radius(ROD_RADIUS),
		CylinderGeometrySetting::Height(ROD_HEIGHT),
		CylinderGeometrySetting::SegmentsRadial(6),
	];
	let rod = create_entity()?
		.set_component_attribute(CylinderGeometry(geometry_settings))?
		.set_component_attribute(material)?
		.set_component_attribute(Position(0.0, -(0.5 * ROD_HEIGHT) + (0.5 * HAND_HEIGHT) + 0.03, 0.0))?
		;
	let rig = create_entity()?
		.set_id(ENTITY_ID)?
		.append_child(rod)?
		.set_component_attribute(Visible::False)?
		;
	Ok(rig)
}

fn get_default_color() -> Color {
	let color = Color::WebStr("Gold");
	color
}

pub fn get_keystaff() -> AEntityEx {
	document().get_element_by_id(ENTITY_ID).unwrap().unchecked_into::<AEntityEx>()
}

pub fn keystaff_reset_color(keystaff: &AEntityEx) {
	keystaff_set_color(keystaff, get_default_color())
}

pub fn keystaff_set_color(keystaff: &AEntityEx, color: Color) {
	let child = keystaff.first_element_child().unwrap().unchecked_into::<AEntityEx>();
	let material = Material::new().set_color(color);
	child.set_component_attribute(material)
}