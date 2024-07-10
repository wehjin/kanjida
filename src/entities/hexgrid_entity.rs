use aframers::components::Position;
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::JsValue;

use crate::components::hexcell_component::attribute::Hexcell;
use crate::hexgrid::HexCoord;
use crate::ka::parse_kanji;

pub fn make() -> Result<Entity, JsValue> {
	let cells = {
		let mut cells = vec![];
		let kanji = parse_kanji();
		for k in kanji {
			let hexcell = Hexcell::new()
				.set_glyph(&k.kanji)
				;
			let entity = create_entity()?.set_component(hexcell)?;
			cells.push(entity);
		};
		cells
	};
	let spiral_coords = HexCoord::ORIGIN.iter_spiral().take(cells.len()).collect::<Vec<_>>();
	let mut grid = create_entity()?;
	for (i, cell) in cells.into_iter().enumerate() {
		let pixel = spiral_coords[i].to_pixel();
		let (x, y) = pixel.flip_y();
		let position = Position(x, y, 0.);
		let cell = cell
			.set_component(position)?
			.set_id(&format!("hexcell-{}", i + 1))?
			;
		grid = grid.append_child(cell)?;
	}
	Ok(grid)
}