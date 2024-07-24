use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::components::Position;
use aframers::entities::Entity;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::aframe_ex::af_sys::AEntityEx;
use crate::aframe_ex::components::material::Material;
use crate::aframe_ex::components::visible_component::Visible;
use crate::aframe_ex::js::log_value;
use crate::ecs::components::hex_color_component::HexColor;
use crate::views::settings::{FOCUS_RING_SELECTOR, FOCUS_RING_Z_OFFSET, SELECT_RING_SELECTOR, SELECT_RING_Z_OFFSET};

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
		let cell_is_focus_ring_target = match focus_ring.get_attribute("target") {
			Some(target) if target == self.a_entity().id() => true,
			_ => false
		};
		let select_ring = self.select_ring_entity();
		let cell_is_select_ring_target = match select_ring.get_attribute("target") {
			Some(target) if target == self.a_entity().id() => true,
			_ => false
		};
		let world = self.a_entity().unchecked_into::<AEntityEx>().world_position_in_new_vector();
		let state = self.ring_color_from_entity_state();
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
				Entity::from(focus_ring)
					.set_component(Material::new().set_color(state.to_color())).unwrap()
					.set_component(position).unwrap()
					.set_component(Visible::True).unwrap()
				;
				select_ring.set_attribute("target", &self.a_entity().id()).unwrap();
				let position = Position(world.x(), world.y(), world.z() + SELECT_RING_Z_OFFSET);
				Entity::from(select_ring)
					.set_component(Material::new().set_color(state.to_color())).unwrap()
					.set_component(position).unwrap()
					.set_component(Visible::True).unwrap()
				;
			}
			HexColor::Selected => {
				select_ring.set_attribute("target", &self.a_entity().id()).unwrap();
				let position = Position(world.x(), world.y(), world.z() + SELECT_RING_Z_OFFSET);
				Entity::from(select_ring)
					.set_component(Material::new().set_color(state.to_color())).unwrap()
					.set_component(position).unwrap()
					.set_component(Visible::True).unwrap()
				;
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
