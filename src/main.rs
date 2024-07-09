use aframers::browser::document;
use aframers::component::Position;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::Closure;
use web_sys::HtmlScriptElement;

use aframe_ex::Scene;
use entities::{camera_entity, chest_entity, ground_entity, hexgrid_entity, light_entity, origin_entity, sky_entity};

use crate::components::{collider_check_component, hexcell_component};
use crate::entities::controller_entity;

pub mod aframe_ex;
pub mod hexgrid;
pub mod ka;

mod components;
mod entities;

fn main() {
	console_error_panic_hook::set_once();
	aframers::init(load).expect("Aframe init");
}

fn load() -> Result<(), JsValue> {
	let element = document().create_element("script")?;
	let script: &HtmlScriptElement = &element.unchecked_ref();
	script.set_type("text/javascript");
	script.set_src("assets/updated-shader.js");
	script.add_event_listener_with_callback(
		"load",
		Closure::once_into_js(|| { run().expect("run"); }).as_ref().unchecked_ref(),
	)?;
	document().head().expect("head").append_child(script)?;
	Ok(())
}

fn run() -> Result<(), JsValue> {
	collider_check_component::register();
	hexcell_component::register();

	let scene = Scene::new()?
		.add_entity(light_entity::make_over()?)?
		.add_entity(light_entity::make_under()?)?
		.add_entity(origin_entity::make()?)?
		.add_entity(ground_entity::make()?)?
		.add_entity(sky_entity::make()?)?
		.add_entity(chest_entity::make()?)?
		.add_entity(
			hexgrid_entity::make()?.set_component(Position(0.0, 3.0, -12.0))?
		)?
		.add_entity(controller_entity::make()?)?
		.add_entity(camera_entity::make()?)?
		;
	document().body().expect("body").append_child(scene.element())?;
	Ok(())
}


