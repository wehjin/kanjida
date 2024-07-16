use aframers::af_sys::entities::a_entity_create;
use aframers::components::{Color, Position, Scale};
use aframers::entities::Entity;
use wasm_bindgen::JsValue;

use crate::aframe_ex::{Align, Baseline, Value};

pub fn make() -> Entity {
	try_make().unwrap()
}

fn try_make() -> Result<Entity, JsValue> {
	let scale = 2.5;
	let entity = Entity::from(a_entity_create("a-text")?)
		.set_component(Align::Center)?
		.set_component(Baseline::Bottom)?
		.set_component(Color::Web("gold".into()))?
		.set_component(Position(0.0, 1.6, -11.0))?
		.set_component(Value("Hello, World!"))?
		.set_component(Scale(scale, scale, scale))?
		;
	Ok(entity)
}
