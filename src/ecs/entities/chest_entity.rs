use aframers::components::Color;
use aframers::entities::{create_box_entity, Entity};
use wasm_bindgen::JsValue;

use crate::ecs::components::yomigun_component::attribute::Yomigun;

pub fn create_chest_entity() -> Result<Entity, JsValue> {
	let chest = create_box_entity()?
		.set_id("yomigun")?
		.set_component(Color::Web("RoyalBlue".into()))?
		.set_component(Yomigun::Enabled)?
		;
	Ok(chest)
}