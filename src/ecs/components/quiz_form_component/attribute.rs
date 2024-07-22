use wasm_bindgen::JsValue;
use crate::aframe_ex::schema::properties::Vec2SchemaProperty;
use crate::aframe_ex::schema::{SchemaProperty, SchemaPropertyType};
use crate::aframe_ex::schema::settings::ComponentAttribute;
use crate::ecs::components::quiz_form_component::COMPONENT_NAME;

#[derive(Debug, Copy, Clone, Default)]
pub struct QuizFormAttribute {
	pub unsolved: usize,
	pub solved: usize,
}

impl SchemaProperty for QuizFormAttribute {
	fn to_schema_property_type(&self) -> SchemaPropertyType {
		SchemaPropertyType::Vec2
	}
	fn to_schema_property_default(&self) -> JsValue {
		Vec2SchemaProperty::create_js(self.unsolved, self.solved)
	}
}

impl ComponentAttribute for QuizFormAttribute {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		COMPONENT_NAME
	}
	fn as_attribute_str(&self) -> impl AsRef<str> {
		Vec2SchemaProperty::format(self.unsolved, self.solved)
	}
}