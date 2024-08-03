use aframers::browser::{document, log};
use aframers::components::{Color, Position};
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Object;

use crate::aframe_ex::components::laser_controls_component::Hand;
use crate::aframe_ex::components::material_component::Material;
use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::geometries::cylinder_geometry::{CylinderGeometry, CylinderGeometrySetting};
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::ecs::fonts::with_kana_font;
use crate::three_sys;
use crate::three_sys::{Mesh, MeshStandardMaterial, TextGeometry};

const ROD_HEIGHT: f32 = 1.3;
const ROD_RADIUS: f32 = 0.020;
const HAND_HEIGHT: f32 = 0.09;

const ROD_ABOVE_HAND: f32 = 0.070;
const ROD_TOP: f32 = (0.5 * HAND_HEIGHT) + ROD_ABOVE_HAND;
const CROWN_HEIGHT: f32 = 0.18;
const CROWN_DEPTH: f32 = 0.010;
const CROWN_GAP: f32 = CROWN_HEIGHT / 5.;
const CROWN_CENTER_Y: f32 = ROD_TOP + CROWN_GAP + (CROWN_HEIGHT / 2.);

pub const CROWN_DEFAULT_GLYPH: &str = "〇";

#[wasm_bindgen]
pub fn set_keystaff_glyph(glyph: &str) -> Result<(), JsValue> {
	let crown_selector = keystaff_crown_selector(&Hand::Right);
	let crown = document().query_selector(&crown_selector)?.unwrap().unchecked_into::<AEntityEx>();
	keystaff_set_crown_glyph(&crown, glyph);
	Ok(())
}

pub fn keystaff_crown_selector(hand: &Hand) -> String {
	format!("#{}", get_keystaff_crown_id(&hand))
}

pub fn keystaff_set_crown_glyph(crown: &AEntityEx, glyph: &str) {
	let own_glyph = glyph.to_string();
	let mesh = crown.object3d().children().get(0).unchecked_into::<Mesh>();
	if mesh.name() != own_glyph {
		with_kana_font(move |font| {
			log(&format!("Updated keystaff crown glyph: {}", &own_glyph));
			mesh.geometry().dispose();
			mesh.set_geometry(&create_crown_geometry(own_glyph.as_str(), font));
			mesh.set_name(own_glyph.as_str());
		});
	}
}

pub fn create_keystaff_entity(hand: &Hand) -> Result<Entity, JsValue> {
	let rig = create_entity()?
		.set_id(get_keystaff_id(hand))?
		.append_child(create_rod()?)?
		.append_child(create_crown(hand)?)?
		.set_component_attribute(Visible::False)?
		;
	Ok(rig)
}

fn get_keystaff_id(hand: &Hand) -> String {
	let keystaff_id = format!("keystaff-{}", hand.as_str());
	keystaff_id
}

fn get_keystaff_crown_id(hand: &Hand) -> String {
	let string = format!("{}-crown", get_keystaff_id(hand));
	string
}

fn create_crown(hand: &Hand) -> Result<Entity, JsValue> {
	let entity = create_entity()?
		.set_id(get_keystaff_crown_id(hand))?
		.set_component_attribute(Position(0.0, CROWN_CENTER_Y, 0.0))?
		;
	let a_entity = entity.a_entity().clone().unchecked_into::<AEntityEx>();
	with_kana_font(move |font| {
		let glyph = CROWN_DEFAULT_GLYPH;
		let geo = create_crown_geometry(glyph, font);
		let mat = MeshStandardMaterial::new();
		mat.set_color(&three_sys::Color::new_str("Gold"));
		mat.set_emissive(&three_sys::Color::new_str("Crimson"));
		let mesh = Mesh::new_with_geometry_and_material(&geo, &mat);
		geo.dispose();
		mat.dispose();
		mesh.set_name(glyph);
		a_entity.object3d().add(&mesh);
	});
	Ok(entity)
}

fn create_crown_geometry(glyph: &str, font: &Object) -> TextGeometry {
	let params = three_sys::TextGeometryParameters::new();
	params.set_font(font);
	params.set_size(CROWN_HEIGHT);
	params.set_depth(CROWN_DEPTH);
	params.set_bevel_thickness(CROWN_DEPTH / 4.);
	params.set_bevel_size(CROWN_DEPTH / 5.);
	params.set_bevel_enabled(true);

	let geo = TextGeometry::new(glyph, params.as_js());
	geo.compute_bounding_box();
	geo.center();
	geo
}

fn create_rod() -> Result<Entity, JsValue> {
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
		.set_component_attribute(Position(0.0, -(0.5 * ROD_HEIGHT) + ROD_TOP, 0.0))?
		;
	Ok(rod)
}

fn get_default_color() -> Color {
	let color = Color::WebStr("Gold");
	color
}

pub fn keystaff_reset_color(keystaff: &AEntityEx) {
	update_staff_color(keystaff, get_default_color())
}

pub fn update_staff_color(keystaff: &AEntityEx, color: Color) {
	let child = keystaff.first_element_child().unwrap().unchecked_into::<AEntityEx>();
	let material = Material::new().set_color(color);
	child.set_component_attribute(material)
}