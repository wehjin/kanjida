use aframers::entities::{create_entity, Entity};
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::laser_controls_component::{Hand, LaserControls};
use crate::aframe_ex::components::raycaster_component::Raycaster;

const TARGETS: &'static str = "[hexcell],[yomigun]";

pub fn make() -> Result<Entity, JsValue> {
	let entity = create_entity()?
		.set_component(LaserControls::new().set_hand(Hand::Right))?
		.set_component(Raycaster::new().set_objects(TARGETS))?
		;
	Ok(entity)
}

