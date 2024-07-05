use aframers::component::{Color, Position};
use aframers::entity::{create_box_entity, Entity};
use wasm_bindgen::JsValue;

use crate::components::collider_check_component::ColliderCheck;

pub fn make() -> Result<Entity, JsValue> {
	let chest = create_box_entity()?
		.set_component(Color::Web("goldenrod"))?
		.set_component(Position(0.0, 0.5, -0.5 - 0.5))?
		.set_component(ColliderCheck)?
		;
	Ok(chest)
}