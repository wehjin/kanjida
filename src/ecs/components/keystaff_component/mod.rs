use aframers::components::Position;
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::oculus_touch_controls_component::OculusTouchControlsEvent::{GripDown, GripUp};
use crate::aframe_ex::components::raycaster_component::{Raycaster, RaycasterSetting};
use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::js::log_value;
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::ecs::components::keystaff_component::bindgen::{KeystaffAComponent, TickTask};
use crate::ecs::entities::keystaff_entity::get_keystaff;
use crate::three_sys::Vector3;

pub const COMPONENT_NAME: &'static str = "keystaff";

pub mod attribute;
pub mod bindgen;

pub fn register_keystaff_component() {
	let events = Events::new()
		.set_handler(GripDown, on_grip_down)
		.set_handler(GripUp, on_grip_up)
		;
	ComponentDefinition::new()
		.set_events(events)
		.set_tick(on_tick)
		.register(COMPONENT_NAME)
	;
}

fn on_tick(comp: KeystaffAComponent, _time: usize, _time_delta: usize) {
	if let Some(tick_task) = comp.take_tick_task() {
		perform_tick_task(&tick_task, comp.a_entity().unchecked_ref());
		comp.set_tick_task(Some(tick_task));
	}
}

fn perform_tick_task(tick_task: &TickTask, controller: &AEntityEx) {
	let pos = &tick_task.pos;
	controller.object3d().get_world_position(&pos);
	tick_task.keystaff.set_component_attribute(Position(pos.x(), pos.y(), pos.z()));
}

fn on_grip_down(comp: KeystaffAComponent, event: CustomEvent) {
	log_value(&event);
	let controller = comp.a_entity().unchecked_into::<AEntityEx>();
	controller.set_component_attribute(Raycaster(vec![
		RaycasterSetting::Enabled(false),
		RaycasterSetting::ShowLine(false),
	]));
	let tick_task = TickTask {
		keystaff: get_keystaff(),
		pos: Vector3::origin(),
	};
	tick_task.keystaff.set_component_attribute(Visible::True);
	perform_tick_task(&tick_task, &controller);
	comp.set_tick_task(Some(tick_task));
}

fn on_grip_up(comp: KeystaffAComponent, event: CustomEvent) {
	log_value(&event);
	let entity = comp.a_entity().unchecked_into::<AEntityEx>();
	entity.set_component_attribute(Raycaster(vec![
		RaycasterSetting::Enabled(true),
		RaycasterSetting::ShowLine(true),
	]));
	let tick_task = comp.take_tick_task().unwrap();
	tick_task.keystaff.set_component_attribute(Visible::False);
	comp.set_tick_task(None);
}