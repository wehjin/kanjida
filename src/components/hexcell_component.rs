use aframers::component::core::{Component, ComponentValue};
use aframers::entity::{create_entity, Entity};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys;

use crate::aframe_ex::{Align, Anchor, Baseline, ComponentDefinition, RingGeometry, Text};

pub struct HexCell {
	glyph: String,
}

impl HexCell {
	pub fn new(glyph: impl AsRef<str>) -> Self {
		Self { glyph: glyph.as_ref().to_string() }
	}
}

impl ComponentValue for HexCell {
	fn component_name(&self) -> &str { "hexcell" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("glyph: {}", &self.glyph)
	}
}


pub fn register() {
	fn schema() -> js_sys::Object {
		let glyph_property = js_sys::Object::new();
		js_sys::Reflect::set(&glyph_property, &"type".into(), &"string".into()).expect("set type");
		js_sys::Reflect::set(&glyph_property, &"default".into(), &"美".into()).expect("set default");

		let schema = js_sys::Object::new();
		js_sys::Reflect::set(&schema, &"glyph".into(), &glyph_property).expect("set glyph");
		schema
	}
	fn init(hexcell: Component) {
		let data = hexcell.data();
		let object: &js_sys::Object = data.as_ref().unchecked_ref();
		let glyph = js_sys::Reflect::get(object, &"glyph".into()).expect("glyph");
		let glyph = glyph.as_string().expect("string");
		let ring = ring_entity(&glyph).expect("make ring");
		let element = hexcell.el();
		element.append_child(ring.element()).expect("append ring");
	}
	ComponentDefinition::new()
		.set_property("schema", &schema())
		.set_init(init)
		.register("hexcell")
	;
}

fn ring_entity(text_value: impl AsRef<str>) -> Result<Entity, JsValue> {
	let geometry = RingGeometry::default()
		.set_segments_theta(6)
		.set_radius_outer(1.0)
		;
	let text = Text::new(text_value)
		.set_font("assets/kanjialive-msdf.json")
		.set_wrap_count(1)
		.set_align(Align::Center)
		.set_anchor(Anchor::Center)
		.set_baseline(Baseline::Center)
		;
	let entity = create_entity()?
		.set_component(geometry)?
		.set_component(text)?
		;
	Ok(entity)
}
