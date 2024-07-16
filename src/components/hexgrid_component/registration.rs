use aframers::af_sys::components::AComponent;
use aframers::components::Position;
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::JsCast;

use crate::components::hexcell_component::attribute::Hexcell;
use crate::components::hexgrid_component::{HexgridAComponent, SelectedEntity};
use crate::GAME;
use crate::game::quiz::Quiz;
use crate::hexgrid::HexCoord;

pub fn init(component: AComponent) -> SelectedEntity {
	let component: HexgridAComponent = component.unchecked_into();
	let mut grid = Entity::from(component.a_entity())
		.set_id("hexgrid-1").unwrap()
		;
	let cells = GAME.with_borrow(|game| game.as_quizzes().into_iter()
		.map(hexcell_entity)
		.collect::<Vec<_>>()
	);
	let spiral_coords = HexCoord::ORIGIN.iter_spiral().take(cells.len()).collect::<Vec<_>>();
	for (i, cell) in cells.into_iter().enumerate() {
		let pixel = spiral_coords[i].to_pixel();
		let (x, y) = pixel.flip_y();
		let position = Position(x, y, 0.);
		let cell = cell.set_component(position).unwrap();
		grid = grid.append_child(cell).unwrap();
	}

	SelectedEntity::none()
}

fn hexcell_entity(quiz: &Quiz) -> Entity {
	let glyph = quiz.question();
	let entity = create_entity().unwrap()
		.set_id(quiz.id()).unwrap()
		.set_component(Hexcell::new().set_glyph(glyph)).unwrap()
		;
	entity
}

pub fn remove(_component: AComponent) {}