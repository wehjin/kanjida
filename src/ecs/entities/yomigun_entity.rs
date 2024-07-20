use aframers::components::Color;
use aframers::entities::{create_plane_entity, Entity};
use wasm_bindgen::JsValue;

use crate::ecs::components::yomigun_component::attribute::Yomigun;
use crate::views::yomi_data::YomiChar;

pub fn create_yomigun() -> Result<Entity, JsValue> {
	let chest = create_plane_entity()?
		.set_id("yomigun")?
		.set_component(Color::Web("Silver".into()))?
		.set_component(Yomigun::from(YomiChar(0).to_code()))?
		;
	Ok(chest)
}