use web_sys::js_sys::Object;

pub trait Schema {
	fn to_object(self) -> Object;
}

pub mod fields;
pub mod multi_property;
pub mod single_property;

