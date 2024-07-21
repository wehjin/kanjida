use aframers::af_sys::components::AComponent;
use aframers::af_sys::entities::AEntity;
use aframers::browser::log;
use aframers::components::core::ComponentValue;
use web_sys::Event;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::cursor_component::CursorEvent::{Click, MouseEnter, MouseLeave};
use crate::aframe_ex::schema::properties::Field;
use crate::aframe_ex::schema::single_property::SinglePropertySchema;
use crate::ecs::components::game_component::GameEvent;
use crate::game::QuizPoint;
use crate::views::quiz_point_from_element_id;

pub const NAME: &'static str = "laserfocus";
pub fn register_laserfocus_component() {
	let events = Events::new()
		.set_handler(MouseEnter, handle_enter)
		.set_handler(MouseLeave, handle_leave)
		.set_handler(Click, handle_click)
		;

	let schema = SinglePropertySchema::from(Field::string(Laserfocus::On));

	ComponentDefinition::new()
		.set_events(events)
		.set_schema(schema)
		.register(NAME);
}

fn handle_click(a_component: AComponent, _event: Event) {
	let entity = a_component.a_entity();
	let quiz_point = quiz_point_from_element_id(entity.id().as_str());
	log(&format!("LASERFOCUS_CLICK: Emit SelectQuiz({})", quiz_point));
	do_effect_emit_select_yomi(quiz_point, entity);
}

fn do_effect_emit_select_yomi(quiz_point: QuizPoint, entity: AEntity) {
	entity.emit_event_with_details(GameEvent::SelectQuiz.as_ref(), &quiz_point.into());
}

fn handle_enter(a_component: AComponent, _event: Event) {
	a_component.a_entity().add_state("focused");
}

fn handle_leave(a_component: AComponent, _event: Event) {
	a_component.a_entity().remove_state("focused");
}

pub enum Laserfocus {
	On
}
impl ComponentValue for Laserfocus {
	fn component_name(&self) -> &str { NAME }

	fn component_value(&self) -> impl AsRef<str> { self }
}
impl AsRef<str> for Laserfocus {
	fn as_ref(&self) -> &str {
		match self {
			Laserfocus::On => "on"
		}
	}
}