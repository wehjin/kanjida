use aframers::components::{Color, Depth, Height, Width};
use aframers::entities::{create_box_entity, Entity};
use wasm_bindgen::JsValue;

pub fn make() -> Result<Entity, JsValue> {
	let origin = create_box_entity()?
		.set_component_attribute(Width(0.05))?
		.set_component_attribute(Height(0.05))?
		.set_component_attribute(Depth(0.05))?
		.set_component_attribute(Color::WebStr("red"))?
		;
	Ok(origin)
}