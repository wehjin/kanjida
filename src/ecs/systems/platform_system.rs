use std::cell::RefCell;

use aframers::browser::document;
use aframers::components::Position;
use wasm_bindgen::JsCast;

use crate::aframe_ex::components::animation_component::{Animation, Easing};
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::ecs::systems::hexgrid_system::hexgrid_system_selected_cell_position;
use crate::run::PLATFORM_ENTITY_ID;

#[derive(Default)]
struct SystemState {
	anim_end_position: Position,
}
impl SystemState {
	pub fn new() -> Self {
		Self { anim_end_position: Position(0., 0., 0.) }
	}
}

thread_local! {
	static SYSTEM_STATE: RefCell<SystemState> = RefCell::new(SystemState::default());
}

pub fn platform_system_animate_to_view_selected_hexcell() {
	let new_anim_end_position = SYSTEM_STATE.with_borrow_mut(|state| match hexgrid_system_selected_cell_position() {
		Some(selected_cell_position) => {
			let goal_position = Position(selected_cell_position.0, selected_cell_position.1 - 1.6, state.anim_end_position.2);
			match state.anim_end_position == goal_position {
				true => None,
				false => {
					state.anim_end_position = goal_position;
					Some(goal_position)
				}
			}
		}
		None => { None }
	});

	if let Some(end_position) = new_anim_end_position {
		let platform = document().get_element_by_id(PLATFORM_ENTITY_ID).unwrap().unchecked_into::<AEntityEx>();
		platform.set_component_attribute(Animation::new()
			.set_property("position")
			.set_to(end_position)
			.set_dur_millis(800)
			.set_delay_millis(20) // No or too short of a delay seems to cause a fault in Quest Browser.
			.set_easing(Easing::EaseInOutQuad)
		);
	}
}