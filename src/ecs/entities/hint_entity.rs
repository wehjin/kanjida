use aframers::af_sys::entities::{a_entity_create, AEntity};
use aframers::browser::document;
use aframers::components::{Color, Position, Scale};
use aframers::entities::Entity;
use wasm_bindgen::{JsCast, JsValue};

use crate::aframe_ex::{Align, Anchor, Baseline};
use crate::aframe_ex::af_sys::AEntityEx;
use crate::aframe_ex::components::font_component::{CustomFont, Negate};
use crate::aframe_ex::components::visible_component::Visible;
use crate::ecs::components::quiz_form_component::quiz_form::QuizForm;
use crate::views::yomi_data::YOMI_FONT;

pub const ENTITY_ID: &'static str = "hintEntity";

pub fn get() -> Entity {
	let element = document().get_element_by_id(ENTITY_ID).unwrap().unchecked_into::<AEntity>();
	Entity::from(element)
}

pub fn create_hint_cursor() -> Result<Entity, JsValue> {
	let scale = 2.5;
	let entity = Entity::from(a_entity_create("a-text")?)
		.set_id(ENTITY_ID)?
		.set_component(Align::Center)?
		.set_component(Baseline::Center)?
		.set_component(Anchor::Align)?
		.set_component(Color::Web("Yellow".into()))?
		.set_component(CustomFont(YOMI_FONT))?
		.set_component(Negate(false))?
		.set_component(Position(0.0, 1.6, -11.0))?
		.set_component(Scale(scale, scale, scale))?
		.set_component(Visible::False)?
		;
	entity.a_entity().unchecked_ref::<AEntityEx>()
		.set_component_attribute(QuizForm { unsolved: 1, solved: 0, revealed: 0 });
	Ok(entity)
}

pub fn get_hint_cursor() -> AEntityEx {
	document().get_element_by_id(ENTITY_ID)
		.unwrap()
		.unchecked_into::<AEntityEx>()
}
