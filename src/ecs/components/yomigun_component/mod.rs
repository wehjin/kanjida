use aframers::components::core::ComponentSetting;

use crate::aframe_ex::components::core::{ComponentDefinition, Events};
use crate::aframe_ex::components::cursor_component::CursorEvent::Click;
use crate::aframe_ex::events::StateEventKind::{StateAdded, StateRemoved};
use crate::aframe_ex::schema::multi_property::MultiPropertySchema;
use crate::ecs::components::yomigun_component::handlers::{yomigun_click, yomigun_state_added, yomigun_state_removed};
use crate::ecs::components::yomigun_component::lifecycle::{init, remove, update};
use crate::ecs::components::yomigun_component::settings::YomigunSetting;

pub const YOMIGUN: &'static str = "yomigun";


pub mod attribute {
	use aframers::components::core::{ComponentAttribute, ComponentSetting};

	use crate::ecs::components::yomigun_component::settings::YomigunSetting;
	use crate::ecs::components::yomigun_component::YOMIGUN;

	#[derive(Debug, Clone)]
	pub struct Yomigun(Vec<YomigunSetting>);

	impl From<usize> for Yomigun {
		fn from(value: usize) -> Self {
			let setting = YomigunSetting::YomiCode(value);
			Self(vec![setting])
		}
	}

	impl ComponentAttribute for Yomigun {
		fn as_attribute_name(&self) -> impl AsRef<str> { YOMIGUN }
		fn as_attribute_str(&self) -> impl AsRef<str> {
			let settings = self.0.iter()
				.map(|setting| setting.as_setting_str().as_ref().to_string())
				.collect::<Vec<_>>()
				.join("; ")
				;
			settings
		}
	}
}

pub fn register_yomigun_component() {
	let events = Events::new()
		.set_handler(StateAdded, yomigun_state_added)
		.set_handler(StateRemoved, yomigun_state_removed)
		.set_handler(Click, yomigun_click)
		;
	let schema = {
		let yomi_code = YomigunSetting::YomiCode(0);
		let setting_name = yomi_code.as_setting_name();
		MultiPropertySchema::new().push(setting_name.as_ref(), yomi_code.to_field())
	};
	ComponentDefinition::new()
		.set_events(events)
		.set_schema(schema)
		.set_init_update_remove(init, update, remove)
		.register("yomigun")
	;
}

mod lifecycle {
	use aframers::af_sys::entities::AEntity;
	use aframers::entities::Entity;

	use crate::ecs::components::yomi_text_component::yomi_text;
	use crate::ecs::components::yomigun_component::bindgen::YomigunAComponent;
	use crate::views::yomi_data::YomiChar;

	pub fn init(this: &YomigunAComponent) {
		let yomi_code = this.yomigun_data().yomi_code();
		Entity::from(this.a_entity())
			.set_component_attribute(yomi_text(YomiChar(yomi_code))).unwrap()
		;
	}
	pub fn update(this: &YomigunAComponent) {
		let yomi_code = this.yomigun_data().yomi_code();
		update_entity(YomiChar(yomi_code), this.a_entity());
	}

	fn update_entity(yomi_char: YomiChar, entity: AEntity) {
		let text_value = yomi_char.as_glyph();
		entity.update_component_property("text", "value", &text_value.into());
	}

	pub fn remove(_this: &YomigunAComponent) {}
}

pub mod handlers {
	use aframers::af_sys::components::AComponent;
	use aframers::af_sys::scenes::AScene;
	use aframers::components::{Color, Scale};
	use aframers::entities::Entity;
	use web_sys::{CustomEvent, Event};

	use crate::aframe_ex::components::cursor_component::CursorState::CursorHovered;
	use crate::aframe_ex::events::StateEvent;
	use crate::aframe_ex::events::StateEventKind::{StateAdded, StateRemoved};
	use crate::aframe_ex::js::log_value;
	use crate::ecs::components::game_component::GameEvent;

	pub fn yomigun_state_added(component: AComponent, js_event: Event) {
		if let Some(event) = StateEvent::try_from_js(&js_event, StateAdded) {
			if event.state() == CursorHovered.as_ref() {
				Entity::from(component.a_entity())
					.set_component_attribute(Scale(1.05, 1.05, 1.05)).unwrap()
					.set_component_attribute(Color::Web("gold".into())).unwrap()
				;
			}
		}
	}
	pub fn yomigun_state_removed(component: AComponent, js_event: Event) {
		if let Some(event) = StateEvent::try_from_js(&js_event, StateRemoved) {
			if event.state() == CursorHovered.as_ref() {
				Entity::from(component.a_entity())
					.set_component_attribute(Scale(1., 1., 1.)).unwrap()
					.set_component_attribute(Color::Web("silver".into())).unwrap()
				;
			}
		}
	}
	pub fn yomigun_click(component: AComponent, js_event: CustomEvent) {
		log_value(&js_event);
		do_emit_select_answer(component.a_entity().a_scene());
	}

	fn do_emit_select_answer(a_scene: AScene) {
		a_scene.emit_event(GameEvent::SubmitAnswer.as_ref());
	}
}
pub mod settings;
pub mod bindgen;
