use aframers::entities::{create_entity, Entity};
use wasm_bindgen::{JsCast, JsValue};

use crate::aframe_ex::components::laser_controls_component::{Hand, LaserControls};
use crate::aframe_ex::components::raycaster_component::Raycaster;
use crate::aframe_ex::js::log_value;
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::ecs::components::keystaff_component::attribute::Keystaff;

const RIGHT_HAND_TARGETS: &'static str = "[hexcell],[yomigun],[yomikey],#answers";

pub fn create_right_controller() -> Result<Entity, JsValue> {
	let entity = create_entity()?
		.set_id("right-control")?
		.set_component_attribute(LaserControls::new().set_hand(Hand::Right))?
		.set_component_attribute(Raycaster::objects(RIGHT_HAND_TARGETS))?
		.set_component_attribute(Keystaff::from(Hand::Right))?
		;
	Ok(entity)
}

pub fn create_left_controller() -> Result<Entity, JsValue> {
	let entity = create_entity()?
		.set_id("left-control")?
		.set_component_attribute(LaserControls::new().set_hand(Hand::Left))?
		.set_component_attribute(Raycaster::enabled(false))?
		.set_component_attribute(Keystaff::from(Hand::Left))?
		;
	let attribute = entity.a_entity().unchecked_ref::<AEntityEx>().get_component_attribute("raycaster");
	log_value(&attribute);
	Ok(entity)
}
