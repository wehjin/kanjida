use aframers::af_sys::components::AComponent;
use aframers::components::Position;
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::JsCast;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::events::StateEventKind;
use crate::aframe_ex::schema::{Field, SinglePropertySchema};
use crate::components::hexcell_component::attribute::Hexcell;
use crate::components::hexgrid_component::{handlers, Hexgrid, HEXGRID_COMPONENT_NAME, HexgridAComponent, SelectedEntity};
use crate::hexgrid::HexCoord;
use crate::ka::parse_kanji;

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

fn init(component: AComponent) -> SelectedEntity {
	let component: HexgridAComponent = component.unchecked_into();
	let mut grid = Entity::from(component.a_entity())
		.set_id("hexgrid-1").unwrap()
		;
	let cells = {
		let mut cells = vec![];
		let kanji = parse_kanji();
		for k in kanji {
			let entity = create_entity().unwrap()
				.set_component(Hexcell::new().set_glyph(&k.kanji)).unwrap()
				;
			cells.push(entity);
		};
		cells
	};
	let spiral_coords = HexCoord::ORIGIN.iter_spiral().take(cells.len()).collect::<Vec<_>>();
	for (i, cell) in cells.into_iter().enumerate() {
		let pixel = spiral_coords[i].to_pixel();
		let (x, y) = pixel.flip_y();
		let position = Position(x, y, 0.);
		let cell = cell
			.set_component(position).unwrap()
			.set_id(&format!("hexcell-{}", i + 1)).unwrap()
			;
		grid = grid.append_child(cell).unwrap();
	}

	SelectedEntity::none()
}

fn remove(_component: AComponent) {}