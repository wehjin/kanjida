use aframers::browser::log;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Reflect;

use crate::aframe_ex::systems::{BuildSystem, register_system, System};

pub struct MySystem<'a>(pub &'a System);
impl<'a> MySystem<'a> {
	pub fn set_property(&mut self, name: impl AsRef<str>, value: &JsValue) {
		Reflect::set(self.0, &name.as_ref().into(), value).expect("set system value");
	}
	pub fn get_property(&self, name: impl AsRef<str>) -> JsValue {
		Reflect::get(self.0, &name.as_ref().into()).expect("get system value")
	}
}

fn init(this: System, _: JsValue) {
	log("init system");
	let mut system = MySystem(&this);
	system.set_property("selectedCell", &JsValue::null());
}

fn select(this: System, selected: JsValue) {
	let mut system = MySystem(&this);
	system.set_property("selectedCell", &selected);
}

pub fn register() {
	let system = BuildSystem::new()
		.set_init(init)
		.add_function("select", select)
		.to_object();
	register_system("cell-selection", &system);
}


