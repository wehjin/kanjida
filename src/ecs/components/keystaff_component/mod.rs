use aframers::components::{Color, Position};
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::laser_controls_component::Hand;
use crate::aframe_ex::components::oculus_touch_controls_component::OculusTouchControlsEvent::{GripDown, GripUp};
use crate::aframe_ex::components::raycaster_component::{Raycaster, RaycasterSetting};
use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::js::log_value;
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::aframe_ex::scenes::A_SCENE;
use crate::ecs::components::game_component::GameEvent::{SelectYomi, SubmitAnswer};
use crate::ecs::components::keystaff_component::attribute::Keystaff;
use crate::ecs::components::keystaff_component::bindgen::{KeystaffAComponent, KeystaffState, TickTask};
use crate::ecs::entities::keystaff_entity::{create_keystaff_entity, CROWN_DEFAULT_GLYPH, get_keystaff_crown_selector, keystaff_reset_color, keystaff_set_color, keystaff_set_crown_glyph};
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
		.set_schema(Keystaff::schema())
		.set_init_remove_ref(on_init, on_remove)
		.set_tick(on_tick)
		.register(COMPONENT_NAME)
	;
}

fn on_init(comp: &KeystaffAComponent) {
	let hand = Keystaff::get_hand(&comp.data());
	let keystaff = create_keystaff_entity(&hand).unwrap()
		.set_component_attribute(Position(0.2, 1.6 - 0.5, -0.2)).unwrap()
		.into_a_entity()
		.unchecked_into::<AEntityEx>()
		;
	A_SCENE.with(|scene| {
		scene.append_child(&keystaff).unwrap()
	});
	comp.set_keystaff_state(KeystaffState { hand, keystaff, tick_task: None });
}
fn on_remove(comp: &KeystaffAComponent) {
	comp.take_keystaff_state();
}

fn on_tick(comp: KeystaffAComponent, _time: usize, _time_delta: usize) {
	let mut state = comp.take_keystaff_state();
	match state.hand {
		Hand::Right => {
			if let Some(tick_task) = &mut state.tick_task {
				let controller = comp.a_entity().unchecked_into::<AEntityEx>();
				let position = controller.compute_world_position(&tick_task.vec3);
				state.keystaff.set_component_attribute(position);
				let index = get_grid_index(&position, &tick_task);
				if index != tick_task.current_index {
					tick_task.current_index = index;
					keystaff_set_color(&state.keystaff, GRID_COLORS[index].clone());
					select_crown(tick_task);
					select_yomi(tick_task);
				}
			}
		}
		Hand::Left => {}
	}
	comp.set_keystaff_state(state);
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
	let mut state = comp.take_keystaff_state();
	match state.hand {
		Hand::Right => {
			let controller = comp.a_entity().unchecked_into::<AEntityEx>();
			controller.set_component_attribute(Raycaster(vec![
				RaycasterSetting::Enabled(false),
				RaycasterSetting::ShowLine(false),
			]));
			let register = Vector3::origin();
			let position = controller.compute_world_position(&register);
			state.keystaff.set_component_attribute(position.clone());
			state.keystaff.set_component_attribute(Visible::True);
			const CELL_RADIUS: f32 = 0.05;
			let tick_task = TickTask {
				vec3: register,
				row2_min: position.2 - CELL_RADIUS,
				row2_max: position.2 + CELL_RADIUS,
				col2_min: position.0 - CELL_RADIUS,
				col2_max: position.0 + CELL_RADIUS,
				crown: state.keystaff.query_selector(&get_keystaff_crown_selector(&state.hand)).unwrap().unwrap().unchecked_into::<AEntityEx>(),
				current_index: 0,
			};
			select_yomi(&tick_task);
			keystaff_set_color(&state.keystaff, GRID_COLORS[tick_task.current_index].clone());
			select_crown(&tick_task);
			state.tick_task = Some(tick_task);
		}
		Hand::Left => {}
	}
	comp.set_keystaff_state(state);
}

fn select_yomi(tick_task: &TickTask) {
	let yomi_point = get_yomi_point(tick_task);
	A_SCENE.with(|scene| {
		scene.emit_event_with_details(SelectYomi.as_ref(), &yomi_point.into());
	})
}

fn on_grip_up(comp: KeystaffAComponent, event: CustomEvent) {
	log_value(&event);
	let controller = comp.a_entity().unchecked_into::<AEntityEx>();
	controller.set_component_attribute(Raycaster(vec![
		RaycasterSetting::Enabled(true),
		RaycasterSetting::ShowLine(true),
	]));
	let mut state = comp.take_keystaff_state();
	state.keystaff.set_component_attribute(Visible::False);
	keystaff_reset_color(&state.keystaff);

	let tick_task = state.tick_task.take().unwrap();
	if tick_task.current_index != 0 {
		A_SCENE.with(|scene| {
			scene.emit_event(SubmitAnswer.as_ref());
		});
	}
	comp.set_keystaff_state(state);
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
