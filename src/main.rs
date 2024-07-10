use aframers::browser::document;
use aframers::components::{Position, Rotation};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::Closure;
use web_sys::HtmlScriptElement;

use aframe_ex::scenes::Scene;
use entities::{camera_entity, chest_entity, ground_entity, hexgrid_entity, light_entity, origin_entity, sky_entity};

use crate::components::hexcell_component;
use crate::entities::controller_entity;
use crate::systems::hexcell_system;

pub mod aframe_ex;
pub mod hexgrid;
pub mod ka;

mod entities;
mod components;
mod systems;

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
	hexcell_system::register();
	hexcell_component::register();

	let scene = Scene::new()?
		.add_entity(light_entity::make_over()?)?
		.add_entity(light_entity::make_under()?)?
		.add_entity(origin_entity::make()?)?
		.add_entity(ground_entity::make()?)?
		.add_entity(sky_entity::make()?)?
		.add_entity(
			chest_entity::make()?
				.set_component(Position(0., -0.25, -1.6))?
				.set_component(Rotation(30., 0., 0.))?
		)?
		.add_entity(
			hexgrid_entity::make()?.set_component(Position(0.0, 3.0, -12.0))?
		)?
		.add_entity(controller_entity::make()?)?
		.add_entity(camera_entity::make()?)?
		;
	document().body().expect("body").append_child(scene.a_scene())?;
	Ok(())
}


