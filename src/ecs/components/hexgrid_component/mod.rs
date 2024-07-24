use ::hexgrid::coordinates::AxialCoord;
use aframers::af_sys::components::AComponent;
use aframers::components::Position;
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::JsCast;

use hexgrid::Hexgrid;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::events::StateEventKind;
use crate::aframe_ex::schema::fields::Field;
use crate::aframe_ex::schema::single_property::SinglePropertySchema;
use crate::ecs::components::hexcell_component::attribute::Hexcell;
use crate::ecs::components::hexgrid_component::bindgen::HexgridAComponent;
use crate::ecs::components::hexgrid_component::other::SelectedEntity;
use crate::GAME;
use crate::game::quiz_state::QuizState;
use crate::views::element_id_from_quiz_point;
use crate::views::settings::PLAIN_RING_Z_OFFSET;

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
	let cells = GAME.with_borrow(|game| {
		game.as_quiz_states()
			.into_iter()
			.enumerate()
			.map(|(quiz_point, quiz_state)| {
				let element_id = element_id_from_quiz_point(quiz_point);
				hexcell_entity(quiz_state).set_id(element_id).unwrap()
			})
			.collect::<Vec<_>>()
	});
	let spiral_coords = AxialCoord::ORIGIN.iter_spiral().take(cells.len()).collect::<Vec<_>>();
	for (i, cell) in cells.into_iter().enumerate() {
		let pixel = spiral_coords[i].to_pixel();
		let (x, y) = pixel.flip_y();
		let position = Position(x, y, PLAIN_RING_Z_OFFSET);
		let cell = cell.set_component(position).unwrap();
		grid = grid.append_child(cell).unwrap();
	}

	SelectedEntity::none()
}

fn hexcell_entity(quiz_state: &QuizState) -> Entity {
	let glyph = quiz_state.as_question();
	let entity = create_entity().unwrap()
		.set_component(Hexcell::new().set_glyph(glyph)).unwrap()
		;
	entity
}

pub fn remove(_component: AComponent) {}