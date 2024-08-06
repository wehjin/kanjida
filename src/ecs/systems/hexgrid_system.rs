use std::cell::RefCell;

use aframers::components::Position;

use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::three_sys::Vector3;

struct HexgridSystemState {
	pub selection: Option<(String, Position)>,
	pub vector3: Vector3,
}
impl HexgridSystemState {
	pub fn new() -> Self {
		Self { selection: None, vector3: Vector3::origin() }
	}
}

thread_local! {
	static STATE : RefCell<HexgridSystemState> = RefCell::new(HexgridSystemState::new());
}


pub fn hexgrid_system_set_selected_cell(entity: &AEntityEx) {
	STATE.with_borrow_mut(|state| {
		let id = entity.id();
		let position = entity.compute_world_position(&state.vector3);
		state.selection = Some((id, position))
	})
}
pub fn hexgrid_system_clear_selected_cell(entity: &AEntityEx) {
	STATE.with_borrow_mut(|state| {
		if let Some((id, _position)) = &state.selection {
			if &entity.id() == id {
				state.selection = None;
			}
		}
	})
}

pub fn hexgrid_system_selected_cell_position() -> Option<Position> {
	STATE.with_borrow(|state| {
		state.selection.clone().map(|(_id, position)| position)
	})
}

