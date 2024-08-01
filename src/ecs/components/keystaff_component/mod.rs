use std::cell::LazyCell;
use std::collections::HashMap;

use aframers::components::{Color, Position};
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::laser_controls_component::Hand;
use crate::aframe_ex::components::oculus_touch_controls_component::OculusTouchControlsEvent::{GripDown, GripUp};
use crate::aframe_ex::components::raycaster_component::{Raycaster, RaycasterSetting};
use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::events::core::AEvent;
use crate::aframe_ex::js::log_value;
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::aframe_ex::scenes::A_SCENE;
use crate::aframe_ex::schema::properties::Vec3SchemaProperty;
use crate::ecs::components::game_component::GameEvent::{SelectYomi, SubmitAnswer};
use crate::ecs::components::keystaff_component::attribute::Keystaff;
use crate::ecs::components::keystaff_component::bindgen::{KeystaffAComponent, KeystaffState, TickTask};
use crate::ecs::entities::keystaff_entity::{create_keystaff_entity, CROWN_DEFAULT_GLYPH, get_keystaff_crown_selector, keystaff_reset_color, keystaff_set_color, keystaff_set_crown_glyph};
use crate::ecs::systems::keystaff_system::ACTIVE_SHIELD;
use crate::ecs::systems::keystaff_system::shield_point::ShieldPoint;
use crate::three_sys::Vector3;
use crate::views::yomi_data::YOMI_BOOK;

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
	let position = match hand {
		Hand::Right => Position(0.2, 1.6 - 0.5, -0.2),
		Hand::Left => Position(-0.2, 1.6 - 0.5, -0.2),
	};
	let keystaff = create_keystaff_entity(&hand).unwrap()
		.set_component_attribute(position).unwrap()
		.into_a_entity().unchecked_into::<AEntityEx>()
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
				let shield_point = get_shield_point(&position, &tick_task);
				if (shield_point != tick_task.current_shield_point()) || !tick_task.has_active_bank() {
					tick_task.set_current_shield_point(shield_point);
					tick_task.update_bank_to_active();
					keystaff_set_color(&state.keystaff, get_rod_color(shield_point));
					update_crown(tick_task);
					select_yomi(tick_task);
				}
				state.keystaff.set_component_attribute(position);
			}
		}
		Hand::Left => {}
	}
	comp.set_keystaff_state(state);
}

fn update_crown(tick_task: &TickTask) {
	let crown_glyph = tick_task.try_glyph().unwrap_or_else(|| CROWN_DEFAULT_GLYPH);
	keystaff_set_crown_glyph(&tick_task.crown, crown_glyph);
}

thread_local! {
	static GRID_COLORS: LazyCell<HashMap<ShieldPoint, Color>> = LazyCell::new(map_points_to_colors)
}

fn map_points_to_colors() -> HashMap<ShieldPoint, Color> {
	const POINTS: [ShieldPoint; 9] = [
		ShieldPoint::LeftBack, ShieldPoint::CenterBack, ShieldPoint::RightBack,
		ShieldPoint::LeftMiddle, ShieldPoint::CenterMiddle, ShieldPoint::RightMiddle,
		ShieldPoint::LeftFront, ShieldPoint::CenterFront, ShieldPoint::RightFront,
	];
	const COLORS: [Color; 9] = [
		Color::WebStr("Red"), Color::WebStr("Orange"), Color::WebStr("Yellow"),
		Color::WebStr("Cyan"), Color::WebStr("Silver"), Color::WebStr("Green"),
		Color::WebStr("Violet"), Color::WebStr("Indigo"), Color::WebStr("Blue"),
	];
	POINTS.into_iter().zip(COLORS).collect()
}


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
			let mut tick_task = TickTask {
				vec3: register,
				row2_min: position.2 - CELL_RADIUS,
				row2_max: position.2 + CELL_RADIUS,
				col2_min: position.0 - CELL_RADIUS,
				col2_max: position.0 + CELL_RADIUS,
				crown: state.keystaff.query_selector(&get_keystaff_crown_selector(&state.hand)).unwrap().unwrap().unchecked_into::<AEntityEx>(),
				bank: ACTIVE_SHIELD.with_borrow(|shield| shield.active_bank()),
			};
			tick_task.set_current_shield_point(ShieldPoint::CenterMiddle);
			select_yomi(&tick_task);
			keystaff_set_color(&state.keystaff, get_rod_color(tick_task.current_shield_point()));
			update_crown(&tick_task);
			state.tick_task = Some(tick_task);
		}
		Hand::Left => {}
	}
	comp.set_keystaff_state(state);
}

fn get_rod_color(point: ShieldPoint) -> Color {
	let color = GRID_COLORS.with(|colors| {
		colors[&point].clone()
	});
	color
}

fn select_yomi(tick_task: &TickTask) {
	if let Some(glyph) = tick_task.try_glyph() {
		if let Some(yomi_char) = YOMI_BOOK.with(|book| book.find_char(glyph).cloned()) {
			SelectYomi.emit_details(&yomi_char.to_code().into());
		}
	}
}

fn on_grip_up(comp: KeystaffAComponent, event: CustomEvent) {
	log_value(&event);
	let mut state = comp.take_keystaff_state();
	match state.hand {
		Hand::Right => {
			let controller = comp.a_entity().unchecked_into::<AEntityEx>();
			controller.set_component_attribute(Raycaster(vec![
				RaycasterSetting::Enabled(true),
				RaycasterSetting::ShowLine(true),
			]));
			state.keystaff.set_component_attribute(Visible::False);
			keystaff_reset_color(&state.keystaff);

			let tick_task = state.tick_task.take().unwrap();
			if tick_task.current_shield_point() != ShieldPoint::CenterMiddle {
				let crown_position = tick_task.crown.compute_world_position(&tick_task.vec3);
				SubmitAnswer.emit_details(&Vec3SchemaProperty::js_from_position(crown_position));
			}
		}
		Hand::Left => {}
	}
	comp.set_keystaff_state(state);
}

fn get_shield_point(&Position(x, _y, z): &Position, tick_task: &TickTask) -> ShieldPoint {
	if z < tick_task.row2_min {
		// Back row
		if x < tick_task.col2_min {
			ShieldPoint::LeftBack
		} else if x > tick_task.col2_max {
			ShieldPoint::RightBack
		} else {
			ShieldPoint::CenterBack
		}
	} else if z > tick_task.row2_max {
		// Front row
		if x < tick_task.col2_min {
			ShieldPoint::LeftFront
		} else if x > tick_task.col2_max {
			ShieldPoint::RightFront
		} else {
			ShieldPoint::CenterFront
		}
	} else {
		// Middle row
		if x < tick_task.col2_min {
			ShieldPoint::LeftMiddle
		} else if x > tick_task.col2_max {
			ShieldPoint::RightMiddle
		} else {
			ShieldPoint::CenterMiddle
		}
	}
}
