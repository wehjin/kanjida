use aframers::entities::{create_entity, Entity};
use wasm_bindgen::JsValue;

use crate::aframe_ex::components::laser_controls_component::{Hand, LaserControls};
use crate::aframe_ex::components::raycaster_component::Raycaster;
use crate::ecs::components::keystaff_component::attribute::Keystaff;

const TARGETS: &'static str = "[hexcell],[yomigun],[yomikey],#answers";

pub fn create_right_controller() -> Result<Entity, JsValue> {
	let entity = create_entity()?
		.set_component_attribute(LaserControls::new().set_hand(Hand::Right))?
		.set_component_attribute(Raycaster::objects(TARGETS))?
		.set_component_attribute(Keystaff)?
		;
	Ok(entity)
}

