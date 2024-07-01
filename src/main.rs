use aframers::browser::document;
use aframers::component::{Color, Depth, Height, Position, Rotation, Width};
use aframers::entity::{create_box_entity, create_camera_entity, create_entity, create_light_entity, create_plane_entity, create_sky_entity, Entity};
use wasm_bindgen::JsValue;

use more_aframe::Scene;

use crate::more_aframe::{Align, Anchor, Baseline, RingGeometry, Text};

pub mod more_aframe;


fn main() {
	console_error_panic_hook::set_once();
	aframers::init(run).expect("Aframe init");
}

fn run() -> Result<(), JsValue> {
	let sky = create_sky_entity()?.set_component(Color::Web("#5C5C5C"))?;
	let ground = ground_entity()?;
	let camera = camera_entity()?;
	let light = light_entity()?;
	let light2 = light_entity_2()?;
	let origin = create_box_entity()?
		.set_component(Width(0.05))?
		.set_component(Height(0.05))?
		.set_component(Depth(0.05))?
		.set_component(Color::Web("red"))?
		;

	let chest = create_box_entity()?
		.set_component(Color::Web("goldenrod"))?
		.set_component(Position(0.0, 0.5, -0.5 - 0.5))?
		;

	let scene = Scene::new()?
		.add_entity(camera)?
		.add_entity(sky)?
		.add_entity(ground)?
		.add_entity(light)?
		.add_entity(light2)?
		.add_entity(chest)?
		.add_entity(origin)?
		.add_entity(ring_entity()?)?
		;
	document().body().ok_or("no body")?.append_child(scene.element())?;
	Ok(())
}

fn ring_entity() -> Result<Entity, JsValue> {
	let geometry = RingGeometry::default()
		.set_segments_theta(6)
		;
	let text = Text::new("非")
		.set_font("assets/kanjialive_01_08-msdf.json")
		.set_wrap_count(1)
		.set_align(Align::Center)
		.set_anchor(Anchor::Center)
		.set_baseline(Baseline::Center)
		;
	let entity = create_entity()?
		.set_component(geometry)?
		.set_component(Position(0.0, 2.0, -2.0))?
		.set_component(text)?
		;
	Ok(entity)
}

fn camera_entity() -> Result<Entity, JsValue> {
	let camera = create_camera_entity()?
		.set_component(Position(0.0, 1.6, 0.0))?
		;
	Ok(camera)
}

fn light_entity() -> Result<Entity, JsValue> {
	let entity = create_light_entity()?
		.set_component(Color::Web("#fff"))?
		.set_component(Position(0.0, 5.0, -0.5))?;
	Ok(entity)
}

fn light_entity_2() -> Result<Entity, JsValue> {
	let entity = create_light_entity()?
		.set_component(Color::Web("#aaa"))?
		.set_component(Position(0., -0.25, 0.5))?
		;
	Ok(entity)
}

fn ground_entity() -> Result<Entity, JsValue> {
	let size = 40.0;
	let entity = create_plane_entity()?
		.set_component(Position(0.0, 0.0, 0.0))?
		.set_component(Rotation(-90.0, 0.0, 0.0))?
		.set_component(Width(size))?
		.set_component(Height(size))?
		.set_component(Color::Web("#7BC8A4"))?;
	Ok(entity)
}

