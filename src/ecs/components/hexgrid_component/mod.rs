use aframers::af_sys::components::AComponent;
use aframers::components::Position;
use aframers::entities::{create_entity, Entity};
use ::hexgrid::coordinates::{AxialCoord, PixelCoord};
use kanji_data::KanjiData;
use wasm_bindgen::JsCast;
use web_sys::js_sys::{Array, Object};

use hexgrid::Hexgrid;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::events::StateEventKind;
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::aframe_ex::schema::fields::Field;
use crate::aframe_ex::schema::single_property::SinglePropertySchema;
use crate::ecs::components::hexcell_component::attribute::Hexcell;
use crate::ecs::components::hexgrid_component::bindgen::HexgridAComponent;
use crate::ecs::components::hexgrid_component::other::SelectedEntity;
use crate::ecs::fonts::with_kanji_font;
use crate::game::states::quiz_state::QuizState;
use crate::three_sys::{merge_geometries, BufferGeometry, Color, Mesh, MeshBasicMaterial, TextGeometry, TextGeometryParameters};
use crate::views::element_id_from_quiz_point;
use crate::views::settings::PLAIN_RING_Z_OFFSET;
use crate::GAME;

pub mod bindgen;
pub mod handlers;
pub mod hexgrid;
pub mod other;

pub const HEXGRID_COMPONENT_NAME: &str = "hexgrid";

pub fn register_hexgrid_component() {
	let events = Events::new()
		.set_handler(StateEventKind::StateAdded, handlers::handle_state_added)
		.set_handler(StateEventKind::StateRemoved, handlers::handle_state_removed)
		;
	let schema = SinglePropertySchema::from(Field::string(Hexgrid::Enabled.as_ref()));
	ComponentDefinition::new()
		.set_events(events)
		.set_schema(schema)
		.set_init_remove_with_extra_state(init, remove)
		.register(HEXGRID_COMPONENT_NAME);
}


pub fn init(component: AComponent) -> SelectedEntity {
	let component: HexgridAComponent = component.unchecked_into();
	let mut grid = Entity::from(component.a_entity())
		.set_id("hexgrid-1").unwrap()
		;
	let cells = GAME.with_borrow(|game| game.as_quiz_states().into_iter()
		.enumerate()
		.map(|(quiz_point, quiz_state)| {
			let element_id = element_id_from_quiz_point(quiz_point);
			hexcell_entity(quiz_state).set_id(element_id).unwrap()
		})
		.collect::<Vec<_>>()
	);
	let spiral_coords = AxialCoord::ORIGIN.iter_spiral().take(cells.len()).collect::<Vec<_>>();
	for (i, cell) in cells.into_iter().enumerate() {
		let pixel = spiral_coords[i].to_pixel();
		let (x, y) = pixel.flip_y();
		let position = Position(x, y, PLAIN_RING_Z_OFFSET);
		let cell = cell.set_component_attribute(position).unwrap();
		grid = grid.append_child(cell).unwrap();
	}
	load_font_create_mesh(spiral_coords, component.a_entity().unchecked_into());
	SelectedEntity::none()
}

fn load_font_create_mesh(coords: Vec<AxialCoord>, entity: AEntityEx) {
	with_kanji_font(move |font| {
		create_mesh_with_font(&coords, &entity, font);
	});
}

fn create_mesh_with_font(coords: &Vec<AxialCoord>, entity: &AEntityEx, font: &Object) {
	let geometry = create_geometry(coords, font);
	let material = create_material();
	let mesh = Mesh::new_with_geometry_and_material(&geometry, &material);
	mesh.set_name("kanji-grid");
	entity.object3d().add(&mesh);
}

fn create_geometry(coords: &Vec<AxialCoord>, font: &Object) -> BufferGeometry {
	let params = create_text_geometry_params(font);
	let array = Array::new_with_length(coords.len() as u32);
	for i in 0..coords.len() {
		let coord = &coords[i];
		let glyph = KanjiData(i).as_glyph();
		let geometry = create_text_geometry(glyph, coord.to_pixel(), &params);
		array.set(i as u32, geometry.unchecked_into());
	}
	merge_geometries(&array, false)
}

fn create_text_geometry_params(font: &Object) -> TextGeometryParameters {
	let params = TextGeometryParameters::new();
	params.set_font(font);
	params.set_size(1.);
	params.set_depth(0.05);
	params.set_curve_segments(4);
	params
}

const KANJI_SCALE: f32 = 0.55;

fn create_text_geometry(glyph: &str, pixel_coord: PixelCoord, params: &TextGeometryParameters) -> BufferGeometry {
	let pixel_coord = pixel_coord.flip_y();
	let text_geometry = TextGeometry::new(glyph, params.as_js());
	let geometry = text_geometry
		.translate(-0.65, -0.5, 0.)
		.scale(KANJI_SCALE, KANJI_SCALE, KANJI_SCALE)
		.translate(pixel_coord.0, pixel_coord.1, 0.)
		;
	geometry
}

fn create_material() -> MeshBasicMaterial {
	let material = MeshBasicMaterial::new();
	material.set_color(&Color::new_str("Silver"));
	material
}

fn hexcell_entity(quiz_state: &QuizState) -> Entity {
	let hexcell = Hexcell::new()
		.set_glyph(quiz_state.as_question())
		.set_solved(quiz_state.is_solved())
		;
	let entity = create_entity().unwrap().set_component_attribute(hexcell).unwrap();
	entity
}

pub fn remove(_component: AComponent) {}