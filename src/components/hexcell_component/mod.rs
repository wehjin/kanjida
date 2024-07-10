use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::components::{Color, Position};
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::{JsCast, JsValue};

use crate::aframe_ex::{Align, Anchor, Baseline, Field, RingGeometry, Schema, Text};
use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::cursor_component::CursorEvent::{MouseEnter, MouseLeave};
use crate::aframe_ex::components::geometry_component::{Circle, Geometry};
use crate::aframe_ex::components::material::Material;
use crate::components::hexcell_component::data::HexcellData;

pub mod attribute;
pub mod data;

pub fn register() {
	ComponentDefinition::new()
		.set_events(events())
		.set_schema(schema())
		.set_init(init)
		.register("hexcell")
	;
}

fn events() -> Events {
	let events = Events::new()
		.set_handler(MouseEnter, handle_enter)
		.set_handler(MouseLeave, handle_leave)
		;
	events
}

fn schema() -> Schema {
	Schema::new()
		.push("glyph", Field::string("美"))
		.push("ring_color", Field::color(Color::Web("silver".into())))
}

fn handle_enter(this: AComponent, _event: JsValue) {
	let target = ring_entity_in_hexcell(this);
	let material = Material::new().set_color(Color::Web("gold".into()));
	Entity::from(target).set_component(material).expect("set material");
}

fn ring_entity_in_hexcell(hexcell: AComponent) -> AEntity {
	hexcell.a_entity().first_element_child().expect("ring element").unchecked_into::<AEntity>()
}

fn handle_leave(this: AComponent, _event: JsValue) {
	let data = this.data().unchecked_into::<HexcellData>();
	let ring_color = Color::Web(data.ring_color());
	let target = ring_entity_in_hexcell(this);
	let material = Material::new().set_color(ring_color);
	Entity::from(target).set_component(material).expect("set material");
}

fn init(this: AComponent) {
	let data: HexcellData = this.data().unchecked_into();
	let ring = ring_entity(&data.glyph(), &data.ring_color()).expect("make ring");
	let geometry = Geometry::<Circle>::new().set_primitive().set_segments(6);
	let material = Material::new()
		.set_transparent(true)
		.set_opacity(0.)
		.set_color(Color::Web("black".into()))
		;
	Entity::from(this.a_entity())
		.append_child(ring).expect("append ring")
		.set_component(material).expect("set material")
		.set_component(geometry).expect("set geometry")
	;
}


fn ring_entity(text_value: impl AsRef<str>, color: impl AsRef<str>) -> Result<Entity, JsValue> {
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
	let material = Material::new().set_color(Color::Web(color.as_ref().into()));
	let entity = create_entity()?
		.set_component(Position(0., 0., -0.01))?
		.set_component(material)?
		.set_component(geometry)?
		.set_component(text)?
		;
	Ok(entity)
}
