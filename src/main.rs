use std::cell::RefCell;

use aframers::browser::document;
use aframers::components::{Position, Rotation};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::Closure;
use web_sys::HtmlScriptElement;

use aframe_ex::scenes::Scene;
use components::hexgrid_component::register_hexgrid_component;
use entities::{camera_entity, chest_entity, ground_entity, hexgrid_entity, light_entity, origin_entity, sky_entity};
use hexcell_component::register_hexcell_component;
use laserfocus_component::register_laserfocus_component;

use crate::aframe_ex::components::visible_component::Visible;
use crate::components::{hexcell_component, laserfocus_component};
use crate::components::hex_color_component::HexColor;
use crate::components::yomigun_component::register_yomigun_component;
use crate::entities::{controller_entity, hint_entity};
use crate::entities::ring_entity::try_ring_entity;
use crate::game::game::Game;

pub mod aframe_ex;
pub mod hexgrid;
pub mod three_sys;
mod entities;
mod game;
mod components;
mod systems;

thread_local! {
	pub static GAME: RefCell<Game> = RefCell::new(Game::with_limit(Some(309)));
}

pub const TEXT_Z_OFFSET: f32 = 0.01;
pub const PLAIN_RING_Z_OFFSET: f32 = 0.02;
pub const SELECT_RING_Z_OFFSET: f32 = 0.03;
pub const SELECT_RING_ID: &'static str = "select-ring";
pub const SELECT_RING_SELECTOR: &'static str = "#select-ring";
pub const FOCUS_RING_Z_OFFSET: f32 = 0.04;
pub const FOCUS_RING_ID: &'static str = "focus-ring";
pub const FOCUS_RING_SELECTOR: &'static str = "#focus-ring";
pub const HINT_Z_OFFSET: f32 = 0.05;

fn main() {
	console_error_panic_hook::set_once();
	aframers::init(load).expect("Aframe init");
}

const INNER_DIV: &str = r#"
<div id="htmlElementWrapper"
     style="width: 512px; height: 512px; position: fixed; left: 0; top: 0; z-index: -2; overflow: hidden">
    <div id="htmlElement"
         style="width:100%; height:100%; background: #F8F8F8; color: #333; font-size: 48px">
        <img src="assets/sample.svg" style="width:100%; height:100%" alt="sample"/>
    </div>
</div>
"#;

fn load() -> Result<(), JsValue> {
	// Adding the htmlElementWrapper div before loading the shader script allows the Meta Quest browser
	// to draw the html-material starting from the first page load.  If the div is added after the
	// script has already loaded, the first page load renders the material as black and only renders it
	// correctly after the page is refreshed.
	let div = document().create_element("div")?;
	div.set_inner_html(INNER_DIV);
	document().body().expect("body").append_child(&div)?;

	let script = document().create_element("script")?;
	{
		let script: &HtmlScriptElement = &script.unchecked_ref();
		script.set_type("text/javascript");
		script.set_src("assets/updated-shader.js");
		script.add_event_listener_with_callback(
			"load",
			Closure::once_into_js(|| { run().expect("run"); }).as_ref().unchecked_ref(),
		)?;
	}
	document().head().expect("head").append_child(&script)?;
	Ok(())
}

fn run() -> Result<(), JsValue> {
	register_laserfocus_component();
	register_hexcell_component();
	register_hexgrid_component();
	register_yomigun_component();

	let focus_ring = try_ring_entity(HexColor::Focused.as_ref())
		.unwrap().set_id(FOCUS_RING_ID)?
		.set_component(Visible::False)?
		;
	let select_ring = try_ring_entity(HexColor::Selected.as_ref())
		.unwrap().set_id(SELECT_RING_ID)?
		.set_component(Visible::False)?
		;

	let scene = Scene::new()?
		.add_entity(light_entity::make_over()?)?
		.add_entity(light_entity::make_under()?)?
		.add_entity(origin_entity::make()?)?
		.add_entity(ground_entity::make()?)?
		.add_entity(sky_entity::make()?)?
		.add_entity(hint_entity::make())?
		.add_entity(focus_ring)?
		.add_entity(select_ring)?
		.add_entity(
			chest_entity::make_chest_entity()?
				.set_component(Position(0., -0.25, -1.6))?
				.set_component(Rotation(30., 0., 0.))?
		)?
		.add_entity(
			hexgrid_entity::create_hexgrid_entity()?.set_component(Position(0.0, 1.6, -12.0))?
		)?
		.add_entity(controller_entity::make()?)?
		.add_entity(camera_entity::make()?)?
		;
	document().body().expect("body").append_child(scene.a_scene())?;
	Ok(())
}


