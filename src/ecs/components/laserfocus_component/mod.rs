use aframers::af_sys::components::AComponent;
use aframers::components::core::ComponentValue;
use web_sys::Event;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::cursor_component::CursorEvent::{Click, MouseEnter, MouseLeave};
use crate::aframe_ex::schema::{Field, SinglePropertySchema};

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
	a_component.a_entity().add_state("selected");
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