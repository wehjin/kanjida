use std::cell::LazyCell;
use std::ops::Deref;

use aframers::af_sys::entities::AEntity;
use aframers::browser::document;
use aframers::entities::Entity;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::{Array, Reflect};

use three_sys::RingGeometry;

use crate::aframe_ex::components::core::{ComponentDefinition, Dependencies, Events};
use crate::aframe_ex::components::geometry_component::{Circle, Geometry};
use crate::aframe_ex::events::StateEventKind::{StateAdded, StateRemoved};
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::aframe_ex::schema::fields::Field;
use crate::aframe_ex::schema::multi_property::MultiPropertySchema;
use crate::ecs::components::hexcell_component::bindgen::HexcellAComponent;
use crate::ecs::components::hexcell_component::handlers::{handle_state_added, handle_state_removed};
use crate::ecs::components::laserfocus_component;
use crate::three_sys;
use crate::three_sys::MeshBasicMaterial;
use crate::three_sys::{merge_geometries, BufferGeometry, Material, Mesh};
use crate::views::settings::STATUS_RING_Z_OFFSET;

pub mod attribute;
pub mod bindgen;
pub mod data;
pub mod handlers;

pub const COMPONENT_NAME: &'static str = "hexcell";

pub fn register_hexcell_component() {
	let dependencies = Dependencies::new(laserfocus_component::NAME);
	let events = Events::new()
		.set_handler(StateAdded, handle_state_added)
		.set_handler(StateRemoved, handle_state_removed)
		;
	let schema = MultiPropertySchema::new()
		.push("glyph", Field::string("美"))
		.push("solved", Field::boolean(false))
		;
	ComponentDefinition::new()
		.set_dependencies(dependencies)
		.set_events(events)
		.set_schema(schema)
		.set_init_ref(init)
		.set_update_ref(update)
		.register(COMPONENT_NAME)
	;
}

thread_local! {
	static RING_GEOMETRY: LazyCell<BufferGeometry> = LazyCell::new(create_ring_geometry);
}
fn create_ring_geometry() -> BufferGeometry {
	let ring_geometry = RingGeometry::new_with_radius_and_segments(
		0.75,
		0.95,
		6,
	).translate(0., 0., STATUS_RING_Z_OFFSET);
	ring_geometry
}
fn get_ring_geometry() -> BufferGeometry {
	RING_GEOMETRY.with(|lazy| lazy.deref().clone())
}

fn init(this: &HexcellAComponent) {
	// Install the geometry first via the component attribute
	// in the entity so that the entity will be recognized as a
	// ray-casting target.
	{
		let geometry = rear_geometry();
		Entity::from(this.a_entity()).set_component_attribute(geometry).unwrap();
	}
	// Update the mesh second after merging the first ray-cast geometry
	// with a second status-indicator geometry.
	{
		let mesh = get_entity_mesh(this.a_entity());
		let array = Array::new_with_length(2);
		array.set(0, mesh.geometry().unchecked_into());
		array.set(1, get_ring_geometry().unchecked_into());
		mesh.set_geometry(&merge_geometries(&array, false));

		mesh.material().dispose();
		mesh.set_material(&ring_material(this));
	}
}

fn update(this: &HexcellAComponent) {
	let mesh = get_entity_mesh(this.a_entity());
	mesh.set_material(&ring_material(this));
}

fn ring_material(this: &HexcellAComponent) -> Material {
	let solved = this.settings().solved();
	let ring_color = if solved { 1 } else { 0 };
	let material = get_or_create_material(ring_color);
	material
}

fn get_entity_mesh(entity: AEntity) -> Mesh {
	let entity_ex = entity.unchecked_into::<AEntityEx>();
	let mesh = entity_ex.get_object3d_kind("mesh").unchecked_into::<Mesh>();
	mesh
}

fn get_or_create_material(ring_status: usize) -> Material {
	const COLORS: [&'static str; 2] = ["#123", "ForestGreen"];
	let name: JsValue = format!("hexcellMaterial{}", ring_status).into();
	let doc = document();
	let material = match Reflect::get(&doc, &name) {
		Ok(material) if !material.is_undefined() => material,
		_ => {
			let material = MeshBasicMaterial::new();
			material.set_color(&three_sys::Color::new_str(COLORS[ring_status % COLORS.len()]));
			Reflect::set(&doc, &name, &material).unwrap();
			material.unchecked_into()
		}
	};
	material.unchecked_into::<Material>()
}

fn rear_geometry() -> Geometry<Circle> {
	let geometry = Geometry::<Circle>::new().set_primitive().set_segments(6);
	geometry
}
