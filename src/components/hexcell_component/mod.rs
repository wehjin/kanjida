use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::components::{Color, Position};
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::aframe_ex::{Align, Anchor, Baseline, RingGeometry, Text};
use crate::aframe_ex::components::core::{ComponentDefinition, Dependencies, Events};
use crate::aframe_ex::components::geometry_component::{Circle, Geometry};
use crate::aframe_ex::components::material::Material;
use crate::aframe_ex::events::StateEventKind::{StateAdded, StateRemoved};
use crate::aframe_ex::schema::{Field, MultiPropertySchema};
use crate::components::hexcell_component::data::HexcellData;
use crate::components::hexcell_component::handlers::{handle_state_added, handle_state_removed};
use crate::components::laserfocus_component;

pub mod attribute;
pub mod data;
pub mod handlers;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AComponent)]
	#[derive(Clone)]
	pub type HexcellAComponent;
}

impl HexcellAComponent {
	pub fn ring_entity(&self) -> AEntity {
		let first_child = self.a_entity().first_element_child().expect("ring element");
		first_child.unchecked_into::<AEntity>()
	}
	pub fn set_ring_color(&self, color: impl AsRef<str>) {
		let target = self.ring_entity();
		let color = Color::Web(color.as_ref().to_string());
		let material = Material::new().set_color(color);
		Entity::from(target).set_component(material).expect("set material");
	}

	pub fn ring_color_from_entity_state(&self) -> impl AsRef<str> {
		let entity = self.a_entity();
		let focused = entity.is_state("focused");
		let selected = entity.is_state("selected");
		let color = match focused {
			true if selected => "#3B7EA1",
			true => "gold",
			false if selected => "#003262",
			false => "silver"
		};
		color.to_string()
	}

	pub fn set_ring_color_from_entity_state(&self) {
		let color = self.ring_color_from_entity_state();
		self.set_ring_color(color);
	}
}

const NAME: &'static str = "hexcell";

pub fn register_hexcell_component() {
	let dependencies = Dependencies::new(laserfocus_component::NAME);
	let events = Events::new()
		.set_handler(StateAdded, handle_state_added)
		.set_handler(StateRemoved, handle_state_removed)
		;
	let schema = MultiPropertySchema::new()
		.push("glyph", Field::string("美"))
		.push("ring_color", Field::color(Color::Web("silver".into())))
		;
	ComponentDefinition::new()
		.set_dependencies(dependencies)
		.set_events(events)
		.set_schema(schema)
		.set_init(init)
		.register(NAME)
	;
}

fn init(this: AComponent) {
	let this = this.unchecked_into::<HexcellAComponent>();
	let data = this.data().unchecked_into::<HexcellData>();
	let glyph = data.glyph();
	let ring_color = this.ring_color_from_entity_state();
	let ring = ring_entity(&glyph, ring_color).expect("make ring");
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
