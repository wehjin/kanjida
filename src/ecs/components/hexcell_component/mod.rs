use aframers::af_sys::components::AComponent;
use aframers::browser::document;
use aframers::components::Color;
use aframers::entities::Entity;
use wasm_bindgen::JsCast;
use web_sys::js_sys::Reflect;

use crate::aframe_ex::af_sys::AEntityEx;
use crate::aframe_ex::components::core::{ComponentDefinition, Dependencies, Events};
use crate::aframe_ex::components::geometry_component::{Circle, Geometry};
use crate::aframe_ex::events::StateEventKind::{StateAdded, StateRemoved};
use crate::aframe_ex::schema::fields::Field;
use crate::aframe_ex::schema::multi_property::MultiPropertySchema;
use crate::ecs::components::hexcell_component::bindgen::HexcellAComponent;
use crate::ecs::components::hexcell_component::handlers::{handle_state_added, handle_state_removed};
use crate::ecs::components::laserfocus_component;
use crate::three_sys;
use crate::three_sys::Mesh;
use crate::three_sys::MeshBasicMaterial;

pub mod attribute;
pub mod bindgen;
pub mod data;
pub mod handlers;

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
