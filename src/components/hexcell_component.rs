use aframers::af_sys::components::AComponent;
use aframers::components::{Color, Position};
use aframers::components::core::ComponentValue;
use aframers::entities::{create_entity, Entity};
use js_sys::{Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys;

use crate::aframe_ex::{Align, Anchor, Baseline, Field, RingGeometry, Schema, Text};
use crate::aframe_ex::components::core::ComponentDefinition;
use crate::aframe_ex::components::geometry_component::{Circle, Geometry};
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
	fn init(this: AComponent) {
		let data = this.data();
		let object: &Object = data.as_ref().unchecked_ref();
		let glyph = Reflect::get(object, &"glyph".into()).expect("glyph");
		let glyph = glyph.as_string().expect("string");
		let ring = ring_entity(&glyph).expect("make ring");
		let geometry = Geometry::<Circle>::new().set_primitive().set_segments(6);
		let material = Material::new()
			.set_transparent(true)
			.set_opacity(0.)
			.set_color(Color::Web("black"))
			;
		Entity::from(this.a_entity())
			.append_child(ring).expect("append ring")
			.set_component(material).expect("set material")
			.set_component(geometry).expect("set geometry")
			.set_component(ColliderCheck).expect("set collider-check")
		;
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
		.set_component(Position(0., 0., -0.01))?
		.set_component(material)?
		.set_component(geometry)?
		.set_component(text)?
		;
	Ok(entity)
}
