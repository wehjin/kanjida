use aframers::browser::document;
use aframers::components::{Position, Rotation, Scale};
use aframers::entities::Entity;
use wasm_bindgen::JsValue;
use web_sys::Element;

use chest_entity::create_chest_entity;
use hexgrid_entity::create_hexgrid;
use hint_entity::create_hint_cursor;

use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::scenes::Scene;
use crate::ecs::components::hex_color_component::HexColor;
use crate::ecs::components::hexcell_component::register_hexcell_component;
use crate::ecs::components::hexgrid_component::register_hexgrid_component;
use crate::ecs::components::laserfocus_component::register_laserfocus_component;
use crate::ecs::components::yomigun_component::register_yomigun_component;
use crate::ecs::entities::{camera_entity, chest_entity, controller_entity, ground_entity, hexgrid_entity, hint_entity, light_entity, origin_entity, sky_entity};
use crate::ecs::entities::answers_entity::create_answers_panel;
use crate::ecs::entities::ring_entity::try_ring_entity;
use crate::views::settings::{FOCUS_RING_ID, SELECT_RING_ID};

pub fn run() -> Result<(), JsValue> {
	register_components();
	let scene = create_scene()?;
	document().body().expect("body").append_child(scene.a_scene())?;
	Ok(())
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
		.add_entity(create_chest_entity()?
			.set_component(Position(0., -0.25, -1.6))?
			.set_component(Rotation(30., 0., 0.))?
		)?
		.add_entity(create_answers_panel()?
			.set_component(Rotation(-60., -40., 0.)).unwrap()
			.set_component(Position(1.4, 0.35, -1.4)).unwrap()
			.set_component(Scale(1.3, 1.3, 1.3)).unwrap()
		)?
		.add_entity(create_hexgrid()?
			.set_component(Position(0.0, 1.6, -12.0))?
		)?
		.add_entity(controller_entity::make()?)?
		.add_entity(camera_entity::make()?)?
		;
	Ok(scene)
}

fn register_components() {
	register_laserfocus_component();
	register_hexcell_component();
	register_hexgrid_component();
	register_yomigun_component();
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
	let img = document().create_element("img")?;
	img.set_id("spiral");
	img.set_attribute("src", "assets/spiral.png")?;
	img.set_attribute("width", "2272")?;
	img.set_attribute("height", "2272")?;
	let assets = document().create_element("a-assets")?;
	assets.append_child(&img)?;
	Ok(assets)
}