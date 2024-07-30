use aframers::browser::document;
use aframers::components::{Color, Position, Rotation, Scale, Width};
use aframers::entities::{create_plane_entity, Entity};
use Color::WebStr;
use wasm_bindgen::JsValue;
use web_sys::Element;

use hexgrid_entity::create_hexgrid;
use hint_entity::create_hint_cursor;
use yomigun_entity::create_yomigun;

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
use crate::ecs::components::laserfocus_component::register_laserfocus_component;
use crate::ecs::components::quiz_form_component::register_quiz_form_component;
use crate::ecs::components::yomigun_component::register_yomigun_component;
use crate::ecs::components::yomikey_component::register_yomikey_component;
use crate::ecs::entities::{camera_entity, controller_entity, ground_entity, hexgrid_entity, hint_entity, light_entity, origin_entity, sky_entity, yomigun_entity};
use crate::ecs::entities::answers_entity::create_answers_panel;
use crate::ecs::entities::ring_entity::try_ring_entity;
use crate::views::settings::{FOCUS_RING_ID, SELECT_RING_ID};
use crate::views::yomi_data::YOMI_FONT;

pub fn register_components() {
	register_quiz_form_component();
	register_laserfocus_component();
	register_hexcell_component();
	register_hexgrid_component();
	register_yomikey_component();
	register_yomigun_component();
	register_game_component();
}

pub fn init_scene() -> Result<Scene, JsValue> {
	let scene = Scene::get()?
		.add_assets(create_assets()?)
		.add_entity(light_entity::make_over()?)?
		.add_entity(origin_entity::make()?)?
		.add_entity(ground_entity::make()?)?
		.add_entity(sky_entity::make()?)?
		.add_entity(create_hint_cursor()?)?
		.add_entity(create_focus_ring()?)?
		.add_entity(create_select_ring()?)?
		.add_entity(create_yomigun()?
			.set_component_attribute(Position(0., 0.3, -1.6))?
			.set_component_attribute(Rotation(-50., 0., 0.))?
		)?
		.add_entity(create_answers_panel()?
			.set_component_attribute(Rotation(-2., -45., 0.)).unwrap()
			.set_component_attribute(Position(1.4, 1.35, -1.4)).unwrap()
			.set_component_attribute(Scale(1.3, 1.3, 1.3)).unwrap()
		)?
		.add_entity(create_hexgrid()?
			.set_component_attribute(Position(0.0, 1.6, -12.0))?
		)?
		.add_entity(controller_entity::make()?)?
		.add_entity(camera_entity::make()?)?
		.set_component_attribute(Game)?
		.set_component_attribute(Stats)?
		.add_entity(create_details_screen())?
		;
	Ok(scene)
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
	let select_ring = try_ring_entity(HexColor::Selected.as_ref())
		.unwrap().set_id(SELECT_RING_ID)?
		.set_component_attribute(Visible::False)?
		;
	Ok(select_ring)
}

fn create_focus_ring() -> Result<Entity, JsValue> {
	let focus_ring = try_ring_entity(HexColor::Focused.as_ref())
		.unwrap().set_id(FOCUS_RING_ID)?
		.set_component_attribute(Visible::False)?
		;
	Ok(focus_ring)
}

fn create_assets() -> Result<Element, JsValue> {
	let assets = document().create_element("a-assets")?;
	Ok(assets)
}