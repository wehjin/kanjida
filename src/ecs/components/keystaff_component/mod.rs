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
use crate::ecs::entities::keystaff_entity::{create_keystaff_entity, CROWN_DEFAULT_GLYPH, keystaff_crown_selector, keystaff_reset_color, keystaff_set_crown_glyph, update_staff_color};
use crate::ecs::systems::keystaff_system::{hand_shield, RIGHT_SHIELD};
use crate::ecs::systems::keystaff_system::shield_bank::ShieldBank;
use crate::ecs::systems::keystaff_system::shield_point::{Floor, RowCol, ShieldPoint};
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
	let vector3 = Vector3::origin();
	let position = match hand {
		Hand::Right => Position(0.2, 1.6 - 0.5, -0.2),
		Hand::Left => Position(-0.2, 1.6 - 0.5, -0.2),
	};
	let keystaff = create_keystaff_entity(&hand).unwrap()
		.set_component_attribute(position).unwrap()
		.into_a_entity().unchecked_into::<AEntityEx>()
		;
	A_SCENE.with(|scene| scene.append_child(&keystaff).unwrap());
	comp.set_keystaff_state(KeystaffState { hand, keystaff, tick_task: None, vector3 });
}
fn on_remove(comp: &KeystaffAComponent) {
	comp.take_keystaff_state();
}

fn on_tick(comp: KeystaffAComponent, _time: usize, _time_delta: usize) {
	let mut state = comp.take_keystaff_state();
	if let Some(tick_task) = &mut state.tick_task {
		let position = comp.to_controller().compute_world_position(&state.vector3);
		let shield_point = get_shield_point(&position, &tick_task);
		if (shield_point != tick_task.current_shield_point()) || !tick_task.has_active_bank() {
			tick_task.update_current_shield_point(shield_point);
			tick_task.update_bank_to_active();
			render_keystaff(&state.keystaff, &tick_task);
			do_effects(&tick_task);
		}
		state.keystaff.set_component_attribute(position);
	}
	comp.set_keystaff_state(state);
}

fn update_crown_glyph(crown: &AEntityEx, glyph: Option<&str>) {
	let crown_glyph = glyph.unwrap_or_else(|| CROWN_DEFAULT_GLYPH);
	keystaff_set_crown_glyph(crown, crown_glyph);
}

thread_local! {
	static GRID_COLORS: LazyCell<HashMap<RowCol, Color>> = LazyCell::new(map_points_to_colors)
}

fn map_points_to_colors() -> HashMap<RowCol, Color> {
	const ROW_COLS: [RowCol; 9] = [
		RowCol::LeftBack, RowCol::CenterBack, RowCol::RightBack,
		RowCol::LeftMiddle, RowCol::CenterMiddle, RowCol::RightMiddle,
		RowCol::LeftFront, RowCol::CenterFront, RowCol::RightFront,
	];
	const COLORS: [Color; 9] = [
		Color::WebStr("Red"), Color::WebStr("Orange"), Color::WebStr("Yellow"),
		Color::WebStr("Cyan"), Color::WebStr("Silver"), Color::WebStr("Green"),
		Color::WebStr("Violet"), Color::WebStr("Indigo"), Color::WebStr("Blue"),
	];
	ROW_COLS.into_iter().zip(COLORS).collect()
}

fn on_grip_down(comp: KeystaffAComponent, event: CustomEvent) {
	log_value(&event);
	let controller = comp.to_controller();
	let mut state = comp.take_keystaff_state();
	let position = controller.compute_world_position(&state.vector3);
	{
		let Position(x, y, z) = position;
		const CELL_RADIUS: f32 = 0.05;
		let mut tick_task = TickTask {
			row2_min: z - CELL_RADIUS,
			row2_max: z + CELL_RADIUS,
			col2_min: x - CELL_RADIUS,
			col2_max: x + CELL_RADIUS,
			floor2_max: y + CELL_RADIUS * 1.5,
			crown: state.keystaff.query_selector(&keystaff_crown_selector(&state.hand)).unwrap().unwrap().unchecked_into::<AEntityEx>(),
			bank: hand_shield(state.hand).with_borrow(|shield| shield.active_bank()),
			hand: state.hand,
		};
		tick_task.update_current_shield_point(ShieldPoint::GroundCenterMiddle);
		render_keystaff(&state.keystaff, &tick_task);
		do_effects(&tick_task);
		state.tick_task = Some(tick_task);
	}
	state.keystaff.set_component_attribute(position);
	state.keystaff.set_component_attribute(Visible::True);
	comp.set_keystaff_state(state);

	controller.set_component_attribute(Raycaster(vec![
		RaycasterSetting::Enabled(false),
		RaycasterSetting::ShowLine(false),
	]));
}

fn on_grip_up(comp: KeystaffAComponent, event: CustomEvent) {
	log_value(&event);
	let controller = comp.to_controller();
	let mut state = comp.take_keystaff_state();
	let tick_task = state.tick_task.take().unwrap();
	if state.hand == Hand::Right {
		if tick_task.current_shield_point() != ShieldPoint::GroundCenterMiddle {
			let crown_position = tick_task.crown.compute_world_position(&state.vector3);
			SubmitAnswer.emit_details(&Vec3SchemaProperty::js_from_position(crown_position));
		}
	}
	keystaff_reset_color(&state.keystaff);
	state.keystaff.set_component_attribute(Visible::False);
	comp.set_keystaff_state(state);

	controller.set_component_attribute(Raycaster(vec![
		RaycasterSetting::Enabled(true),
		RaycasterSetting::ShowLine(true),
	]));
}

fn do_effects(tick_task: &TickTask) {
	match tick_task.hand {
		Hand::Right => {
			select_yomi(tick_task.try_glyph());
		}
		Hand::Left => {
			let new_bank = match tick_task.current_shield_point().to_row_col() {
				RowCol::LeftBack => ShieldBank::A,
				RowCol::CenterBack => ShieldBank::K,
				RowCol::RightBack => ShieldBank::S,
				RowCol::LeftMiddle => ShieldBank::T,
				RowCol::CenterMiddle => ShieldBank::N,
				RowCol::RightMiddle => ShieldBank::H,
				RowCol::LeftFront => ShieldBank::M,
				RowCol::CenterFront => ShieldBank::Y,
				RowCol::RightFront => ShieldBank::R,
			};
			RIGHT_SHIELD.with_borrow_mut(|shield| shield.set_active_bank(new_bank));
		}
	}
}

fn render_keystaff(keystaff: &AEntityEx, tick_task: &TickTask) {
	update_crown_glyph(&tick_task.crown, tick_task.try_glyph());
	update_staff_color(keystaff, get_rod_color(tick_task.current_shield_point()));
}

fn get_rod_color(point: ShieldPoint) -> Color {
	let row_col = point.to_row_col();
	GRID_COLORS.with(|colors| colors[&row_col].clone())
}

fn select_yomi(option: Option<&str>) {
	if let Some(glyph) = option {
		if let Some(yomi_char) = YOMI_BOOK.with(|book| book.find_char(glyph).cloned()) {
			SelectYomi.emit_details(&yomi_char.to_code().into());
		}
	}
}

fn get_shield_point(&Position(x, y, z): &Position, tick_task: &TickTask) -> ShieldPoint {
	let row_col = if z < tick_task.row2_min {
		// Back row
		if x < tick_task.col2_min {
			RowCol::LeftBack
		} else if x > tick_task.col2_max {
			RowCol::RightBack
		} else {
			RowCol::CenterBack
		}
	} else if z > tick_task.row2_max {
		// Front row
		if x < tick_task.col2_min {
			RowCol::LeftFront
		} else if x > tick_task.col2_max {
			RowCol::RightFront
		} else {
			RowCol::CenterFront
		}
	} else {
		// Middle row
		if x < tick_task.col2_min {
			RowCol::LeftMiddle
		} else if x > tick_task.col2_max {
			RowCol::RightMiddle
		} else {
			RowCol::CenterMiddle
		}
	};
	let floor = if y > tick_task.floor2_max { Floor::Over } else { Floor::Ground };
	row_col.to_point_on_floor(floor)
}
