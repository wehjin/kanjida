use aframers::entities::{create_entity, Entity};
use wasm_bindgen::JsValue;

use crate::components::hexgrid_component::Hexgrid;

pub fn create_hexgrid_entity() -> Result<Entity, JsValue> {
	create_entity()?.set_component(Hexgrid::Enabled)
}