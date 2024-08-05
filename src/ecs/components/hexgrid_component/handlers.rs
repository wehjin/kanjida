use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::components::Position;
use wasm_bindgen::JsCast;
use web_sys::{CustomEvent, Event};

use crate::aframe_ex::scene_entity_bindgen::{AEntityEx, ASceneEx};
use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::events::StateEvent;
use crate::ecs::components::hexgrid_component::bindgen::HexgridAComponent;
use crate::ecs::entities::hint_entity;
use crate::three_sys::Vector3;
use crate::views::settings::HINT_Z_OFFSET;

pub fn handle_state_added(component: AComponent, event: Event) {
	let component = component.unchecked_into::<HexgridAComponent>();
	let custom = event.unchecked_into::<CustomEvent>();
	if let Some(state_added) = StateEvent::try_added(&custom) {
		match state_added.state() {
			"selected" => {
				let cell = custom.target().unwrap().unchecked_into::<AEntity>();
				let cell_world_position = cell.unchecked_ref::<AEntityEx>().world_position_in_new_vector();
				update_hint_entity(&cell_world_position);
				update_component_with_selected_entity_notifying_old(&component, &cell);
				let scene = component.a_entity().a_scene().unchecked_into::<ASceneEx>();
				scene.set_yomisprite_target_position(Some(cell_world_position));
			}
			_ => ()
		}
	}
}

fn update_hint_entity(center: &Vector3) {
	hint_entity::get()
		.set_component_attribute(Position(center.x(), center.y(), center.z() + HINT_Z_OFFSET)).unwrap()
		.set_component_attribute(Visible::True).unwrap()
	;
}

pub fn handle_state_removed(component: AComponent, event: Event) {
	let component = component.unchecked_into::<HexgridAComponent>();
	let custom = event.unchecked_into::<CustomEvent>();
	if let Some(state_removed) = StateEvent::try_removed(&custom) {
		match state_removed.state() {
			"selected" => {
				let target = custom.target().unwrap().unchecked_into::<AEntity>();
				let was_selected = update_component_with_unselected_entity(&component, &target);
				if was_selected {
					let scene = component.a_entity().a_scene().unchecked_into::<ASceneEx>();
					scene.set_yomisprite_target_position(None);
				}
			}
			_ => ()
		}
	}
}

pub fn update_component_with_selected_entity_notifying_old(component: &HexgridAComponent, entity: &AEntity) {
	let mut selected_entity = component.selected_entity();
	let old_entity = selected_entity.replace_entity(entity);
	component.set_selected_entity(selected_entity);
	if let Some(old_entity) = old_entity {
		old_entity.remove_state("selected");
	}
}


/// Return true if the entity was the selected entity.
fn update_component_with_unselected_entity(component: &HexgridAComponent, entity: &AEntity) -> bool {
	let mut selected_entity = component.selected_entity();
	let was_selected = selected_entity.remove_entity_if_selected(entity);
	component.set_selected_entity(selected_entity);
	was_selected
}
