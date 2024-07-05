use aframers::component::Color;
use aframers::component::core::{Component, ComponentValue};
use aframers::entity::{create_entity, Entity};
use js_sys::{Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys;

use crate::aframe_ex::{Align, Anchor, Baseline, ComponentDefinition, Field, RingGeometry, Schema, Text};
use crate::aframe_ex::components::material::Material;
use crate::components::collider_check_component::ColliderCheck;

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
	fn init(hexcell: Component) {
		let data = hexcell.data();
		let object: &Object = data.as_ref().unchecked_ref();
		let glyph = Reflect::get(object, &"glyph".into()).expect("glyph");
		let glyph = glyph.as_string().expect("string");
		let ring = ring_entity(&glyph).expect("make ring");
		let element = hexcell.el();
		element.append_child(ring.element()).expect("append ring");
	}
	ComponentDefinition::new()
		.set_schema(Schema::new().push("glyph", Field::string("美")))
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
	let material = Material::new().set_color(Color::Web("silver"));
	let entity = create_entity()?
		.set_component(material)?
		.set_component(geometry)?
		.set_component(text)?
		.set_component(ColliderCheck)?
		;
	Ok(entity)
}
