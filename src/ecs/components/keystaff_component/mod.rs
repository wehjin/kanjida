use aframers::components::{Color, Position};
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::oculus_touch_controls_component::OculusTouchControlsEvent::{GripDown, GripUp};
use crate::aframe_ex::components::raycaster_component::{Raycaster, RaycasterSetting};
use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::js::log_value;
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::ecs::components::game_component::GameEvent::{SelectYomi, SubmitAnswer};
use crate::ecs::components::keystaff_component::bindgen::{KeystaffAComponent, TickTask};
use crate::ecs::entities::keystaff_entity::{CROWN_DEFAULT_GLYPH, CROWN_SELECTOR, get_keystaff, keystaff_reset_color, keystaff_set_color, keystaff_set_crown_glyph};
use crate::three_sys::Vector3;
use crate::views::yomi_data::YomiChar;

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
	if let Some(mut tick_task) = comp.take_tick_task() {
		let controller = comp.a_entity().unchecked_into::<AEntityEx>();
		// Get the controller's current world position into vec3.
		let vec3 = &tick_task.vec3;
		controller.object3d().get_world_position(&vec3);
		let position = Position(vec3.x(), vec3.y(), vec3.z());
		// Move the keystaff to the controller's current position.
		tick_task.keystaff.set_component_attribute(position);
		// Update the keystaff's properties depending on where it is in relation to the start position.
		let index = get_grid_index(&position, &tick_task);
		if index != tick_task.current_index {
			tick_task.current_index = index;
			keystaff_set_color(&tick_task.keystaff, GRID_COLORS[index].clone());
			select_crown(&mut tick_task);
			select_yomi(&tick_task);
		}
		comp.set_tick_task(Some(tick_task));
	}
}

fn select_crown(tick_task: &TickTask) {
	let glyph = YomiChar(get_yomi_point(tick_task)).as_glyph();
	keystaff_set_crown_glyph(&tick_task.crown, glyph);
}

fn get_yomi_point(tick_task: &TickTask) -> usize {
	// Temporary until we get the correct mapping.
	let yomi_point = tick_task.current_index; // Temporary until we get the correct mapping.
	yomi_point
}

const GRID_GLYPHS: [&str; 9] = [
	CROWN_DEFAULT_GLYPH, "タ", "ア", "イ", "ウ", "エ", "オ", "カ", "サ",
];

const GRID_COLORS: [Color; 9] = [
	Color::WebStr("Silver"), Color::WebStr("Red"), Color::WebStr("Orange"),
	Color::WebStr("Yellow"), Color::WebStr("Green"), Color::WebStr("Blue"),
	Color::WebStr("Indigo"), Color::WebStr("Violet"), Color::WebStr("Cyan"),
];


fn on_grip_down(comp: KeystaffAComponent, event: CustomEvent) {
	log_value(&event);
	let controller = comp.a_entity().unchecked_into::<AEntityEx>();
	controller.set_component_attribute(Raycaster(vec![
		RaycasterSetting::Enabled(false),
		RaycasterSetting::ShowLine(false),
	]));
	let keystaff = get_keystaff();
	let vec3 = Vector3::origin();
	controller.object3d().get_world_position(&vec3);
	let position = Position(vec3.x(), vec3.y(), vec3.z());
	keystaff.set_component_attribute(position.clone());
	keystaff.set_component_attribute(Visible::True);

	const CELL_RADIUS: f32 = 0.05;
	let crown = keystaff.query_selector(CROWN_SELECTOR).unwrap().unwrap().unchecked_into::<AEntityEx>();
	let tick_task = TickTask {
		keystaff,
		vec3,
		row2_min: position.2 - CELL_RADIUS,
		row2_max: position.2 + CELL_RADIUS,
		col2_min: position.0 - CELL_RADIUS,
		col2_max: position.0 + CELL_RADIUS,
		crown,
		current_index: 0,
	};
	select_yomi(&tick_task);
	keystaff_set_color(&tick_task.keystaff, GRID_COLORS[tick_task.current_index].clone());
	select_crown(&tick_task);
	comp.set_tick_task(Some(tick_task));
}

fn select_yomi(tick_task: &TickTask) {
	let yomi_point = get_yomi_point(tick_task);
	tick_task.keystaff.emit_event_with_details(SelectYomi.as_ref(), &yomi_point.into());
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
	keystaff_reset_color(&tick_task.keystaff);
	if tick_task.current_index != 0 {
		tick_task.keystaff.emit_event(SubmitAnswer.as_ref());
	}
	comp.set_tick_task(None);
}

fn get_grid_index(position: &Position, tick_task: &TickTask) -> usize {
	let index = if position.2 < tick_task.row2_min {
		if position.0 < tick_task.col2_min {
			7
		} else if position.0 > tick_task.col2_max {
			1
		} else {
			8
		}
	} else if position.2 > tick_task.row2_max {
		if position.0 < tick_task.col2_min {
			5
		} else if position.0 > tick_task.col2_max {
			3
		} else {
			4
		}
	} else {
		if position.0 < tick_task.col2_min {
			6
		} else if position.0 > tick_task.col2_max {
			2
		} else {
			0
		}
	};
	index
}
