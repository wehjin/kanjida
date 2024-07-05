use aframers::entity::{create_entity, Entity};
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::laser_controls_component::{Hand, LaserControls};

pub fn make() -> Result<Entity, JsValue> {
	let entity = create_entity()?
		.set_component(LaserControls::new().set_hand(Hand::Right))?
		;
	Ok(entity)
}

