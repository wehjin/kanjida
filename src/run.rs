use aframers::components::{Color, Position, Rotation, Width};
use aframers::entities::{create_entity, create_plane_entity, create_sky_entity, Entity};
use controller_entity::create_right_controller;
use hexgrid_entity::create_hexgrid;
use hint_entity::create_hint_cursor;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use Color::WebStr;

use crate::aframe_ex::components::align_component::Align;
use crate::aframe_ex::components::anchor_component::Anchor;
use crate::aframe_ex::components::baseline_component::Baseline;
use crate::aframe_ex::components::stats_component::Stats;
use crate::aframe_ex::components::text_component::Text;
use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::scenes::Scene;
use crate::ecs::components::game_component::game::Game;
use crate::ecs::components::game_component::register_game_component;
use crate::ecs::components::hex_color_component::HexColor;
use crate::ecs::components::hexcell_component::register_hexcell_component;
use crate::ecs::components::hexgrid_component::register_hexgrid_component;
use crate::ecs::components::keystaff_component::register_keystaff_component;
use crate::ecs::components::laserfocus_component::register_laserfocus_component;
use crate::ecs::components::quiz_form_component::register_quiz_form_component;
use crate::ecs::entities::controller_entity::create_left_controller;
use crate::ecs::entities::ring_entity::try_ring_entity;
use crate::ecs::entities::{camera_entity, controller_entity, ground_entity, hexgrid_entity, hint_entity, light_entity, origin_entity};
use crate::views::settings::{FOCUS_RING_ID, SELECT_RING_ID};
use crate::views::yomi_data::YOMI_FONT;

pub const PLATFORM_ENTITY_ID: &'static str = "platform";

pub fn register_components() {
	register_quiz_form_component();
	register_laserfocus_component();
	register_hexcell_component();
	register_hexgrid_component();
	register_game_component();
	register_keystaff_component();
}

#[wasm_bindgen]
pub fn init_scene() -> Result<(), JsValue> {
	Scene::get()?
		.set_component_attribute(Game)?
		.set_component_attribute(Stats)?
		.add_entity(create_sky_entity()?.set_component_attribute(WebStr("#5C5C5C"))?)?
		.add_entity(create_hexgrid()?.set_component_attribute(Position(0.0, 1.6, -12.0))?)?
		.add_entity(create_hint_cursor()?)?
		.add_entity(create_focus_ring()?)?
		.add_entity(create_select_ring()?)?
		.add_entity(create_platform_entity()?)?
	;
	Ok(())
}

fn create_platform_entity() -> Result<Entity, JsValue> {
	let vehicle = create_entity()?
		.set_id(PLATFORM_ENTITY_ID)?
		.append_child(light_entity::make_over()?)?
		.append_child(origin_entity::make()?)?
		.append_child(ground_entity::make()?)?
		.append_child(create_right_controller()?)?
		.append_child(create_left_controller()?)?
		.append_child(camera_entity::make()?)?
		.append_child(create_details_screen())?
		.set_component_attribute(Position(0., 0., 0.))?
		;
	Ok(vehicle)
}

fn create_details_screen() -> Entity {
	let text = Text::new()
		.set_font(YOMI_FONT)
		.set_baseline(Baseline::Center)
		.set_align(Align::Left)
		.set_anchor(Anchor::Center)
		.set_width(Width(0.9))
		.set_color(WebStr("DarkSlateGray"))
		;
	let entity = create_plane_entity().unwrap()
		.set_id("details").unwrap()
		.set_component_attribute(WebStr("Honeydew")).unwrap()
		.set_component_attribute(Position(-1.2, 0.9, -1.2)).unwrap()
		.set_component_attribute(Rotation(-30., 40., 0.)).unwrap()
		.set_component_attribute(text).unwrap()
		;
	entity
}
pub const DETAILS_SCREEN_SELECTOR: &'static str = "#details";

fn create_select_ring() -> Result<Entity, JsValue> {
	let select_ring = try_ring_entity(HexColor::Selected.as_ref()).unwrap()
		.set_id(SELECT_RING_ID)?
		.set_component_attribute(Visible::False)?
		;
	Ok(select_ring)
}

fn create_focus_ring() -> Result<Entity, JsValue> {
	let focus_ring = try_ring_entity(HexColor::Focused.as_ref()).unwrap()
		.set_id(FOCUS_RING_ID)?
		.set_component_attribute(Visible::False)?
		;
	Ok(focus_ring)
}

