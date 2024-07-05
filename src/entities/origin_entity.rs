use aframers::entity::{create_box_entity, Entity};
use wasm_bindgen::JsValue;
use aframers::component::{Color, Depth, Height, Width};

pub fn make() -> Result<Entity, JsValue> {
	let origin = create_box_entity()?
		.set_component(Width(0.05))?
		.set_component(Height(0.05))?
		.set_component(Depth(0.05))?
		.set_component(Color::Web("red"))?
		;
	Ok(origin)
}