use aframers::components::core::ComponentAttribute;
use wasm_bindgen::JsValue;

use crate::aframe_ex::schema::{SchemaProperty, SchemaPropertyType};
use crate::aframe_ex::schema::properties::Vec3SchemaProperty;
use crate::ecs::components::quiz_form_component::COMPONENT_NAME;

#[derive(Debug, Copy, Clone, Default)]
pub struct QuizForm {
	pub unsolved: usize,
	pub solved: usize,
	pub revealed: usize,
}

impl SchemaProperty for QuizForm {
	fn to_schema_property_type(&self) -> SchemaPropertyType {
		SchemaPropertyType::Vec3
	}
	fn format(&self) -> String {
		Vec3SchemaProperty::format_usize(self.unsolved, self.solved, self.revealed)
	}
	fn create_js(&self) -> JsValue {
		Vec3SchemaProperty::create_js_usize(self.unsolved, self.solved, self.revealed)
	}
	fn parse_js(data: &JsValue) -> Self {
		let (x, y, z) = Vec3SchemaProperty::parse_js_usize(data);
		Self { unsolved: x, solved: y, revealed: z }
	}
}

impl ComponentAttribute for QuizForm {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		COMPONENT_NAME
	}
	fn as_attribute_str(&self) -> impl AsRef<str> { self.format() }
}