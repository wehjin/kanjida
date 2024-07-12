use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::events::StateEventKind::{StateAdded, StateRemoved};
use crate::aframe_ex::schema::SinglePropertySchema;
use crate::components::yomigun_component::attribute::Yomigun;
use crate::components::yomigun_component::handlers::{yomigun_state_added, yomigun_state_removed};

pub const YOMIGUN: &'static str = "yomigun";

pub mod attribute {
	use aframers::components::core::ComponentValue;

	use crate::components::yomigun_component::YOMIGUN;

	pub enum Yomigun { Enabled }

	impl AsRef<str> for Yomigun {
		fn as_ref(&self) -> &str {
			match self {
				Yomigun::Enabled => "enabled",
			}
		}
	}
	impl ComponentValue for Yomigun {
		fn component_name(&self) -> &str { YOMIGUN }
		fn component_value(&self) -> impl AsRef<str> { self }
	}
}

pub mod handlers {
	use aframers::af_sys::components::AComponent;
	use aframers::components::{Color, Scale};
	use aframers::entities::Entity;
	use wasm_bindgen::JsValue;

	use crate::aframe_ex::components::cursor_component::CursorState::CursorHovered;
	use crate::aframe_ex::events::StateEvent;
	use crate::aframe_ex::events::StateEventKind::{StateAdded, StateRemoved};

	pub fn yomigun_state_added(component: AComponent, js_event: JsValue) {
		if let Some(event) = StateEvent::try_from_js(&js_event, StateAdded) {
			if event.state() == CursorHovered.as_ref() {
				Entity::from(component.a_entity())
					.set_component(Scale(1.05, 1.05, 1.05)).unwrap()
					.set_component(Color::Web("gold".into())).unwrap()
				;
			}
		}
	}
	pub fn yomigun_state_removed(component: AComponent, js_event: JsValue) {
		if let Some(event) = StateEvent::try_from_js(&js_event, StateRemoved) {
			if event.state() == CursorHovered.as_ref() {
				Entity::from(component.a_entity())
					.set_component(Scale(1., 1., 1.)).unwrap()
					.set_component(Color::Web("silver".into())).unwrap()
				;
			}
		}
	}
}

pub fn register_yomigun_component() {
	let events = Events::new()
		.set_handler(StateAdded, yomigun_state_added)
		.set_handler(StateRemoved, yomigun_state_removed)
		;
	let schema = SinglePropertySchema::from(Yomigun::Enabled);
	ComponentDefinition::new()
		.set_events(events)
		.set_schema(schema)
		.register("yomigun")
	;
}