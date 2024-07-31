use aframers::af_sys::components::AComponent;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::three_sys::Vector3;

#[wasm_bindgen]
pub struct TickTask {
	pub(crate) keystaff: AEntityEx,
	pub(crate) vec3: Vector3,
	pub(crate) row2_min: f32,
	pub(crate) row2_max: f32,
	pub(crate) col2_min: f32,
	pub(crate) col2_max: f32,
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AComponent)]
	pub type KeystaffAComponent;

	#[wasm_bindgen(method, getter, js_name = tickTask)]
	pub fn take_tick_task(this: &KeystaffAComponent) -> Option<TickTask>;
	#[wasm_bindgen(method, setter, js_name = tickTask)]
	pub fn set_tick_task(this: &KeystaffAComponent, tick_task: Option<TickTask>);
}
