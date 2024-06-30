use aframers::browser::document;
use aframers::component::{Color, Depth, Height, Position, Rotation, Width};
use aframers::entity::{create_box_entity, create_camera_entity, create_entity, create_light_entity, create_plane_entity, create_sky_entity, Entity};
use aframers::scene::create_scene;
use wasm_bindgen::JsValue;
use web_sys::Element;

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
		;
	document().body().ok_or("no body")?.append_child(scene.element())?;
	Ok(())
}

fn camera_entity() -> Result<Entity, JsValue> {
	let camera = create_camera_entity()?
		.set_component(Position(0.0, 0.0, 0.0))?
		;
	let rig = create_entity()?
		.set_component(Position(0.0, 3.2, 0.0))?
		.append_child(camera)?
		;
	Ok(rig)
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

pub struct Scene(Element);
impl Scene {
	pub fn new() -> Result<Self, JsValue> {
		let element = create_scene()?;
		let scene = Self(element);
		Ok(scene)
	}
	pub fn add_entity(self, entity: Entity) -> Result<Self, JsValue> {
		self.0.append_child(entity.element())?;
		Ok(self)
	}
	pub fn element(&self) -> &Element {
		&self.0
	}
}