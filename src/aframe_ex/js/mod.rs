use aframers::af_sys::components::AComponent;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Function;

use crate::aframe_ex::systems::ASystem;

#[wasm_bindgen(
	inline_js = "\
	export function to_init(c) { return function () { c(this); }; }\
	export function withFirstFromThis(f) {\
	  return function (...lastArgs) {\
	      f(this,...lastArgs);\
	  };\
	}\
	export function bind_this_to_first(f) {\
		return function (...args) { f(this,...args); };\
	}\
	"
)]
extern "C" {
	pub fn to_init(closure: &Closure<dyn Fn(AComponent)>) -> Function;
	#[wasm_bindgen(js_name = withFirstFromThis)]
	pub fn with_system_from_this(closure: &Closure<dyn Fn(ASystem, JsValue)>) -> Function;
	#[wasm_bindgen]
	pub fn bind_this_to_first(f: Function) -> Function;
}

pub fn bind_this_to_component<T>(f: impl Fn(T, JsValue) + 'static) -> Function
where
	T: AsRef<AComponent> + FromWasmAbi + 'static,
{
	let closure = Closure::wrap(Box::new(f) as Box<dyn Fn(T, JsValue)>);
	let unbound = closure.into_js_value().unchecked_into::<Function>();
	let bound = bind_this_to_first(unbound);
	bound
}
