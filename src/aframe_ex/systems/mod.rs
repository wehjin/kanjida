use aframers::af_sys::scenes::AScene;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Object;

use crate::aframe_ex::js::with_system_from_this;
use crate::aframe_ex::objects::BuildObject;

pub struct BuildSystem {
	init: Option<Box<dyn Fn(ASystem, JsValue) + 'static>>,
	functions: Vec<(String, Box<dyn Fn(ASystem, JsValue) + 'static>)>,
}

impl BuildSystem {
	pub fn new() -> Self {
		Self { init: None, functions: Vec::new() }
	}
	pub fn set_init(self, value: impl Fn(ASystem, JsValue) + 'static) -> Self {
		Self { init: Some(Box::new(value)), ..self }
	}
	pub fn add_function(mut self, name: impl AsRef<str>, value: impl Fn(ASystem, JsValue) + 'static) -> Self {
		self.functions.push((name.as_ref().to_string(), Box::new(value)));
		self
	}
	pub fn to_object(self) -> Object {
		let builder = BuildObject::new().if_option_set_property("init", &self.init.map(to_function_value));
		let builder = self.functions.into_iter().fold(
			builder,
			|builder, (name, function)| {
				let value = to_function_value(function);
				builder.set_property(&name, &value)
			},
		);
		builder.to_object()
	}
}
fn to_function_value(f: Box<dyn Fn(ASystem, JsValue)>) -> JsValue {
	let closure = Closure::wrap(f);
	let function = with_system_from_this(&closure);
	closure.forget();
	function.unchecked_into::<JsValue>()
}

#[wasm_bindgen(module = "/js/rust_systems.js")]
extern "C" {
	pub fn aframe_system_def(rust_system: JsValue) -> Object;
}


#[wasm_bindgen]
extern "C" {
	pub type ASystem;
	#[wasm_bindgen(method, getter)]
	pub fn data(this: &ASystem) -> JsValue;
	#[wasm_bindgen(method, getter, js_name = el)]
	pub fn a_scene(this: &ASystem) -> AScene;
}

#[wasm_bindgen(js_namespace = AFRAME)]
extern "C" {
	#[wasm_bindgen(js_name = registerSystem)]
	pub fn register_system(name: &str, definition: &Object);
}
