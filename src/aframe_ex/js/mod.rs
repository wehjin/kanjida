use aframers::component::core::Component;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Function;

use crate::aframe_ex::systems::System;

#[wasm_bindgen(
	inline_js = "\
	export function to_init(c) { return function () { c(this); }; }\
	export function withFirstFromThis(f) {\
	  return function (...lastArgs) {\
	      f(this,...lastArgs);\
	  };\
	}\
	"
)]
extern "C" {
	pub fn to_init(closure: &Closure<dyn Fn(Component)>) -> Function;
	#[wasm_bindgen(js_name = withFirstFromThis)]
	pub fn with_component_from_this(closure: &Closure<dyn Fn(Component, JsValue)>) -> Function;
	#[wasm_bindgen(js_name = withFirstFromThis)]
	pub fn with_system_from_this(closure: &Closure<dyn Fn(System, JsValue)>) -> Function;
}
