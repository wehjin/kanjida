use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::{Closure, wasm_bindgen};
use web_sys::js_sys::Function;

#[wasm_bindgen]
pub struct EventListener(Closure<dyn Fn(JsValue)>);

impl EventListener {
	pub fn new(handler: impl Fn(JsValue) + 'static) -> Self {
		let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn(JsValue)>);
		Self(closure)
	}
	pub fn as_function(&self) -> &Function {
		self.0.as_ref().unchecked_ref()
	}
}
