use aframers::browser::document;
use aframers::components::{Position, Rotation, Scale};
use aframers::entities::Entity;
use wasm_bindgen::JsValue;
use web_sys::Element;

use hexgrid_entity::create_hexgrid;
use hint_entity::create_hint_cursor;
use yomigun_entity::create_yomigun;

use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::scenes::Scene;
use crate::ecs::components::game_component::{Game, register_game_component};
use crate::ecs::components::hex_color_component::HexColor;
use crate::ecs::components::hexcell_component::register_hexcell_component;
use crate::ecs::components::hexgrid_component::register_hexgrid_component;
use crate::ecs::components::laserfocus_component::register_laserfocus_component;
use crate::ecs::components::quiz_form_component::register_quiz_form_component;
use crate::ecs::components::yomigun_component::register_yomigun_component;
use crate::ecs::components::yomikey_component::register_yomikey_component;
use crate::ecs::entities::{camera_entity, controller_entity, ground_entity, hexgrid_entity, hint_entity, light_entity, origin_entity, sky_entity, yomigun_entity};
use crate::ecs::entities::answers_entity::create_answers_panel;
use crate::ecs::entities::ring_entity::try_ring_entity;
use crate::views::settings::{FOCUS_RING_ID, SELECT_RING_ID};

pub fn run() -> Result<(), JsValue> {
	register_components();
	let scene = create_scene()?;
	document().body().expect("body").append_child(scene.a_scene())?;
	Ok(())
}

fn register_components() {
	register_quiz_form_component();
	register_laserfocus_component();
	register_hexcell_component();
	register_hexgrid_component();
	register_yomikey_component();
	register_yomigun_component();
	register_game_component();
}

fn create_scene() -> Result<Scene, JsValue> {
	let scene = Scene::new()?
		.add_assets(create_assets()?)
		.add_entity(light_entity::make_over()?)?
		.add_entity(origin_entity::make()?)?
		.add_entity(ground_entity::make()?)?
		.add_entity(sky_entity::make()?)?
		.add_entity(create_hint_cursor()?)?
		.add_entity(create_focus_ring()?)?
		.add_entity(create_select_ring()?)?
		.add_entity(create_yomigun()?
			.set_component(Position(0., 0.3, -1.6))?
			.set_component(Rotation(-50., 0., 0.))?
		)?
		.add_entity(create_answers_panel()?
			.set_component(Rotation(-2., -45., 0.)).unwrap()
			.set_component(Position(1.4, 1.35, -1.4)).unwrap()
			.set_component(Scale(1.3, 1.3, 1.3)).unwrap()
		)?
		.add_entity(create_hexgrid()?
			.set_component(Position(0.0, 1.6, -12.0))?
		)?
		.add_entity(controller_entity::make()?)?
		.add_entity(camera_entity::make()?)?
		.set_component(Game)?
		;
	Ok(scene)
}

fn create_select_ring() -> Result<Entity, JsValue> {
	let select_ring = try_ring_entity(HexColor::Selected.as_ref())
		.unwrap().set_id(SELECT_RING_ID)?
		.set_component(Visible::False)?
		;
	Ok(select_ring)
}

fn create_focus_ring() -> Result<Entity, JsValue> {
	let focus_ring = try_ring_entity(HexColor::Focused.as_ref())
		.unwrap().set_id(FOCUS_RING_ID)?
		.set_component(Visible::False)?
		;
	Ok(focus_ring)
}

fn create_assets() -> Result<Element, JsValue> {
	let assets = document().create_element("a-assets")?;
	Ok(assets)
}