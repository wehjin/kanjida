use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::cursor_component::CursorEvent::Click;
use crate::aframe_ex::events::StateEventKind::{StateAdded, StateRemoved};
use crate::aframe_ex::schema::SinglePropertySchema;
use crate::components::yomigun_component::attribute::Yomigun;
use crate::components::yomigun_component::handlers::{yomigun_click, yomigun_state_added, yomigun_state_removed};

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
	use aframers::browser;
	use aframers::components::{Color, Position, Scale};
	use aframers::entities::{create_box_entity, Entity};
	use wasm_bindgen::{JsCast, JsValue};

	use crate::aframe_ex::af_sys::{AEntityEx, ASceneEx};
	use crate::aframe_ex::components::animation_component::{Animation, Easing};
	use crate::aframe_ex::components::cursor_component::CursorState::CursorHovered;
	use crate::aframe_ex::events::StateEvent;
	use crate::aframe_ex::events::StateEventKind::{StateAdded, StateRemoved};
	use crate::aframe_ex::scenes::Scene;

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
	pub fn yomigun_click(component: AComponent, _js_event: JsValue) {
		let yomigun = component.a_entity().unchecked_into::<AEntityEx>();
		let scene = Scene::from(yomigun.a_scene());
		let yomigun_target_vector = scene.a_scene().unchecked_ref::<ASceneEx>().yomigun_target_position();
		browser::log(&format!("yomigun target vector: {:?}", &yomigun_target_vector));
		if let Some(target) = yomigun_target_vector {
			let end = Position(target.x(), target.y(), target.z());
			let start = {
				let relative_position = Position(0., 0., -1.);
				yomigun.local_position_to_world(relative_position)
			};
			let animation = Animation::new()
				.set_property("position")
				.set_from(start)
				.set_to(end)
				.set_dur_millis(100)
				.set_easing(Easing::Linear)
				;
			let sprite = create_sprite_entity(start).set_component(animation).unwrap();
			scene.add_entity(sprite).unwrap();
		}
	}

	fn create_sprite_entity(position: Position) -> Entity {
		const SPRITE_SCALE: f32 = 0.6;
		create_box_entity().unwrap()
			.set_component(Scale(SPRITE_SCALE, SPRITE_SCALE, SPRITE_SCALE)).unwrap()
			.set_component(position).unwrap()
			.set_component(Color::Web("tomato".into())).unwrap()
	}
}

pub fn register_yomigun_component() {
	let events = Events::new()
		.set_handler(StateAdded, yomigun_state_added)
		.set_handler(StateRemoved, yomigun_state_removed)
		.set_handler(Click, yomigun_click)
		;
	let schema = SinglePropertySchema::from(Yomigun::Enabled);
	ComponentDefinition::new()
		.set_events(events)
		.set_schema(schema)
		.register("yomigun")
	;
}