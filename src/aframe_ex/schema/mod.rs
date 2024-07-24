use wasm_bindgen::JsValue;
use web_sys::js_sys::Object;

pub trait Schema {
	fn to_object(self) -> Object;
}

pub enum SchemaPropertyType {
	Usize,
	Vec2,
}
impl SchemaPropertyType {
	pub fn as_str(&self) -> &'static str {
		match self {
			SchemaPropertyType::Vec2 => "vec2",
			SchemaPropertyType::Usize => "int",
		}
	}
}

pub trait SchemaProperty {
	fn to_schema_property_type(&self) -> SchemaPropertyType;
	fn format(&self) -> String;
	fn create_js(&self) -> JsValue;
	fn parse_js(data: &JsValue) -> Self;
}

pub mod fields;
pub mod multi_property;
pub mod properties;
pub mod settings;
pub mod single_property;

