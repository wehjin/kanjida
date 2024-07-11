use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::components::core::ComponentValue;
use aframers::components::Position;
use aframers::entities::{create_entity, Entity};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::CustomEvent;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::events::{StateEvent, StateEventKind};
use crate::aframe_ex::schema::{Field, SinglePropertySchema};
use crate::components::hexcell_component::attribute::Hexcell;
use crate::hexgrid::HexCoord;
use crate::ka::parse_kanji;

pub const HEXGRID_COMPONENT_NAME: &str = "hexgrid";

pub enum Hexgrid {
	Enabled,
}

impl AsRef<str> for Hexgrid {
	fn as_ref(&self) -> &str {
		match self { Hexgrid::Enabled => "enabled" }
	}
}

impl ComponentValue for Hexgrid {
	fn component_name(&self) -> &str { HEXGRID_COMPONENT_NAME }
	fn component_value(&self) -> impl AsRef<str> { "enabled" }
}

#[wasm_bindgen]
#[derive(Clone)]
struct SelectedEntity {
	entity: Option<AEntity>,
}
#[wasm_bindgen]
impl SelectedEntity {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self { entity: None }
	}
	pub fn replace_entity(&mut self, value: &AEntity) -> Option<AEntity> {
		match &self.entity {
			None => {
				self.entity = Some(value.clone());
				None
			}
			Some(old_value) if old_value.id() == value.id() => {
				None
			}
			Some(old_value) => {
				let old_value = old_value.clone();
				self.entity = Some(value.clone());
				Some(old_value)
			}
		}
	}
	pub fn remove_entity_if_present(&mut self, value: &AEntity) {
		if let Some(existing) = &self.entity {
			if existing.id() == value.id() {
				self.entity = None;
			}
		}
	}
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AComponent)]
	pub type HexgridAComponent;

	#[wasm_bindgen(method, getter)]
	pub fn selected_entity(this: &HexgridAComponent) -> SelectedEntity;

	#[wasm_bindgen(method, setter)]
	pub fn set_selected_entity(this: &HexgridAComponent, value: SelectedEntity);
}

impl HexgridAComponent {
	pub fn update_selected_entity_notifying_old(&self, entity: &AEntity) {
		let mut selected_entity = self.selected_entity();
		let old = selected_entity.replace_entity(entity);
		self.set_selected_entity(selected_entity);
		if let Some(old) = old {
			old.remove_state("selected");
		}
	}
	pub fn update_unselected_entity(&self, entity: &AEntity) {
		let mut selected_entity = self.selected_entity();
		selected_entity.remove_entity_if_present(entity);
		self.set_selected_entity(selected_entity);
	}
}

pub fn register_hexgrid_component() {
	let events = Events::new()
		.set_handler(StateEventKind::StateAdded, handle_state_added)
		.set_handler(StateEventKind::StateRemoved, handle_state_removed)
		;
	let schema = SinglePropertySchema::from(Field::string(Hexgrid::Enabled.as_ref()));
	ComponentDefinition::new()
		.set_events(events)
		.set_schema(schema)
		.set_init(init)
		.register(HEXGRID_COMPONENT_NAME);
}

fn handle_state_added(component: AComponent, event: JsValue) {
	let component = component.unchecked_into::<HexgridAComponent>();
	let custom = event.unchecked_into::<CustomEvent>();
	if let Some(state_added) = StateEvent::try_added(&custom) {
		match state_added.state() {
			"selected" => {
				let target = custom.target().unwrap().unchecked_into::<AEntity>();
				component.update_selected_entity_notifying_old(&target);
			}
			_ => ()
		}
	}
}
fn handle_state_removed(component: AComponent, event: JsValue) {
	let component = component.unchecked_into::<HexgridAComponent>();
	let custom = event.unchecked_into::<CustomEvent>();
	if let Some(state_removed) = StateEvent::try_removed(&custom) {
		match state_removed.state() {
			"selected" => {
				let target = custom.target().unwrap().unchecked_into::<AEntity>();
				component.update_unselected_entity(&target);
			}
			_ => ()
		}
	}
}

fn init(component: AComponent) {
	let component = component.unchecked_into::<HexgridAComponent>();
	component.set_selected_entity(SelectedEntity::new());

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
}

