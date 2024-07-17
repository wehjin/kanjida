use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::components::Position;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::CustomEvent;

use crate::{GAME, HINT_Z_OFFSET};
use crate::aframe_ex::af_sys::{AEntityEx, ASceneEx};
use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::events::StateEvent;
use crate::aframe_ex::Value;
use crate::components::hexgrid_component::HexgridAComponent;
use crate::entities::hint_entity;
use crate::three_sys::Vector3;

pub fn handle_state_added(component: AComponent, event: JsValue) {
	let component = component.unchecked_into::<HexgridAComponent>();
	let custom = event.unchecked_into::<CustomEvent>();
	if let Some(state_added) = StateEvent::try_added(&custom) {
		match state_added.state() {
			"selected" => {
				let cell = custom.target().unwrap().unchecked_into::<AEntity>();
				let cell_world_position = cell.unchecked_ref::<AEntityEx>().world_position_in_new_vector();
				update_hint_entity(&cell_world_position, cell.id().as_str());
				update_component_with_selected_entity_notifying_old(&component, &cell);
				let scene = component.a_entity().a_scene().unchecked_into::<ASceneEx>();
				scene.set_yomigun_target_position(Some(cell_world_position));
			}
			_ => ()
		}
	}
}

fn update_hint_entity(center: &Vector3, quiz_id: &str) {
	let hint = GAME.with_borrow(|game| {
		let quiz = game.as_quiz_by_id(quiz_id);
		let hint = quiz.hint().to_string();
		hint
	});
	hint_entity::get()
		.set_component(Position(center.x(), center.y() + 0.6, center.z() + HINT_Z_OFFSET)).unwrap()
		.set_component(Value(hint.to_uppercase())).unwrap()
		.set_component(Visible::True).unwrap()
	;
}

pub fn handle_state_removed(component: AComponent, event: JsValue) {
	let component = component.unchecked_into::<HexgridAComponent>();
	let custom = event.unchecked_into::<CustomEvent>();
	if let Some(state_removed) = StateEvent::try_removed(&custom) {
		match state_removed.state() {
			"selected" => {
				let target = custom.target().unwrap().unchecked_into::<AEntity>();
				let was_selected = update_component_with_unselected_entity(&component, &target);
				if was_selected {
					let scene = component.a_entity().a_scene().unchecked_into::<ASceneEx>();
					scene.set_yomigun_target_position(None);
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
