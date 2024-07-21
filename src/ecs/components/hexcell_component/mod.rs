use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::browser;
use aframers::browser::document;
use aframers::components::{Color, Position};
use aframers::components::core::ComponentValue;
use aframers::entities::Entity;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Reflect;

use crate::three_sys;
use crate::aframe_ex::af_sys::AEntityEx;
use crate::aframe_ex::components::core::{ComponentDefinition, Dependencies, Events};
use crate::aframe_ex::components::geometry_component::{Circle, Geometry};
use crate::aframe_ex::components::material::Material;
use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::events::StateEventKind::{StateAdded, StateRemoved};
use crate::aframe_ex::js::log_value;
use crate::aframe_ex::schema::properties::Field;
use crate::aframe_ex::schema::multi_property::MultiPropertySchema;
use crate::ecs::components::hex_color_component::HexColor;
use crate::ecs::components::hexcell_component::handlers::{handle_state_added, handle_state_removed};
use crate::ecs::components::laserfocus_component;
use crate::three_sys::material::MeshBasicMaterial;
use crate::three_sys::mesh::Mesh;
use crate::views::settings::{FOCUS_RING_SELECTOR, FOCUS_RING_Z_OFFSET, SELECT_RING_SELECTOR, SELECT_RING_Z_OFFSET};

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
	pub fn select_ring_entity(&self) -> AEntity {
		let element = self.a_entity().a_scene().query_selector(SELECT_RING_SELECTOR).unwrap().unwrap();
		element.unchecked_into()
	}
	pub fn focus_ring_entity(&self) -> AEntity {
		let element = self.a_entity().a_scene().query_selector(FOCUS_RING_SELECTOR).unwrap().unwrap();
		element.unchecked_into()
	}

	pub fn ring_color_from_entity_state(&self) -> HexColor {
		let entity = self.a_entity();
		let focused = entity.is_state("focused");
		let selected = entity.is_state("selected");
		let color = if focused {
			if selected {
				HexColor::FocusedAndSelected
			} else {
				HexColor::Focused
			}
		} else {
			if selected {
				HexColor::Selected
			} else {
				HexColor::NeitherFocusedNorSelected
			}
		};
		color
	}

	pub fn set_ring_color_from_entity_state(&self) {
		log_value(&self.a_entity().id().into());
		let focus_ring = self.focus_ring_entity();
		browser::log(&format!("fr id: {}", focus_ring.id()));
		let cell_is_focus_ring_target = match focus_ring.get_attribute("target") {
			Some(target) if target == self.a_entity().id() => true,
			_ => false
		};
		let select_ring = self.select_ring_entity();
		browser::log(&format!("sr id: {}", select_ring.id()));
		let cell_is_select_ring_target = match select_ring.get_attribute("target") {
			Some(target) if target == self.a_entity().id() => true,
			_ => false
		};
		let world = self.a_entity().unchecked_into::<AEntityEx>().world_position_in_new_vector();
		let state = self.ring_color_from_entity_state();
		browser::log(&format!("HexState: {:?}", state));
		match state {
			HexColor::Focused => {
				focus_ring.set_attribute("target", &self.a_entity().id()).unwrap();
				Entity::from(focus_ring)
					.set_component(Material::new().set_color(state.to_color())).unwrap()
					.set_component(Position(world.x(), world.y(), world.z() + FOCUS_RING_Z_OFFSET)).unwrap()
					.set_component(Visible::True).unwrap()
				;
			}
			HexColor::FocusedAndSelected => {
				focus_ring.set_attribute("target", &self.a_entity().id()).unwrap();
				let position = Position(world.x(), world.y(), world.z() + FOCUS_RING_Z_OFFSET);
				browser::log(&format!("fr position: {}", position.component_value().as_ref()));
				Entity::from(focus_ring)
					.set_component(Material::new().set_color(state.to_color())).unwrap()
					.set_component(position).unwrap()
					.set_component(Visible::True).unwrap()
				;
				select_ring.set_attribute("target", &self.a_entity().id()).unwrap();
				let position = Position(world.x(), world.y(), world.z() + SELECT_RING_Z_OFFSET);
				browser::log(&format!("sr position: {}", position.component_value().as_ref()));
				Entity::from(select_ring)
					.set_component(Material::new().set_color(state.to_color())).unwrap()
					.set_component(position).unwrap()
					.set_component(Visible::True).unwrap()
				;
			}
			HexColor::Selected => {
				select_ring.set_attribute("target", &self.a_entity().id()).unwrap();
				browser::log("set the attribute");
				let position = Position(world.x(), world.y(), world.z() + SELECT_RING_Z_OFFSET);
				browser::log(&format!("sr position: {}", position.component_value().as_ref()));
				Entity::from(select_ring)
					.set_component(Material::new().set_color(state.to_color())).unwrap()
					.set_component(position).unwrap()
					.set_component(Visible::True).unwrap()
				;
				browser::log("updated the select ring");
			}
			HexColor::NeitherFocusedNorSelected => {
				if cell_is_focus_ring_target {
					Entity::from(focus_ring.unchecked_into::<AEntity>())
						.set_component(Visible::False).unwrap()
					;
				}
				if cell_is_select_ring_target {
					Entity::from(select_ring.unchecked_into::<AEntity>())
						.set_component(Visible::False).unwrap()
					;
				}
			}
		}
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
	Entity::from(this.a_entity())
		.set_component(rear_geometry()).unwrap()
	;
	let material = {
		let doc = document();
		let material = match Reflect::get(&doc, &"hexcellMaterial".into()) {
			Ok(material) if !material.is_undefined() => material,
			_ => {
				let material = MeshBasicMaterial::new();
				let color = three_sys::Color::new_str("Fuchsia");
				material.set_color(&color);
				Reflect::set(&doc, &"hexcellMaterial".into(), &material).unwrap();
				material.unchecked_into()
			}
		};
		material.unchecked_into::<three_sys::Material>()
	};
	let mesh = this.a_entity().unchecked_into::<AEntityEx>().get_object3d_kind("mesh").unchecked_into::<Mesh>();
	mesh.set_material(&material);
}

fn rear_geometry() -> Geometry<Circle> {
	let geometry = Geometry::<Circle>::new().set_primitive().set_segments(6);
	geometry
}

// fn text_component(text_value: impl AsRef<str> + Sized) -> Text {
// 	let text = Text::new()
// 		.set_align(Align::Center)
// 		.set_anchor(Anchor::Center)
// 		.set_baseline(Baseline::Center)
// 		.set_font("assets/kanjialive-msdf.json")
// 		.set_value(text_value)
// 		.set_wrap_count(1)
// 		.set_z_offset(TEXT_Z_OFFSET)
// 		;
// 	text
// }
