use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::components::{Color, Position};
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::aframe_ex::{Align, Anchor, Baseline, RingGeometry, Text};
use crate::aframe_ex::components::core::{ComponentDefinition, Dependencies, Events};
use crate::aframe_ex::components::cursor_component::CursorEvent::{Click, MouseEnter, MouseLeave};
use crate::aframe_ex::components::geometry_component::{Circle, Geometry};
use crate::aframe_ex::components::material::Material;
use crate::aframe_ex::schema::{Field, MultiPropertySchema};
use crate::components::hexcell_component::data::HexcellData;
use crate::components::hexcell_component::handlers::{handle_click, handle_enter, handle_leave};
use crate::components::laserfocus_component;
use crate::systems::hexcell_system;
use crate::systems::hexcell_system::HexcellASystem;

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

	pub fn ring_color_from_system(&self) -> String {
		let system = self.get_system(hexcell_system::NAME).unchecked_into::<HexcellASystem>();
		system.ring_color(&self.a_entity())
	}

	pub fn restore_ring_color(&self) {
		let ring_color = Color::Web(self.ring_color_from_system());
		let target = self.ring_entity();
		let material = Material::new().set_color(ring_color);
		Entity::from(target).set_component(material).expect("set material");
	}
}

const NAME: &'static str = "hexcell";

pub fn register() {
	let dependencies = Dependencies::new(laserfocus_component::NAME);
	let events = Events::new()
		.set_handler(MouseEnter, handle_enter)
		.set_handler(MouseLeave, handle_leave)
		.set_handler(Click, handle_click)
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
	let data = this.data().unchecked_into::<HexcellData>();
	let glyph = data.glyph();
	let ring_color = api_ring_color(&this);
	let ring = ring_entity(&glyph, &ring_color).expect("make ring");
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

fn api_ring_color(a_component: &AComponent) -> String {
	let cell_system: HexcellASystem = a_component.get_system(hexcell_system::NAME).unchecked_into();
	cell_system.ring_color(&a_component.a_entity())
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
