use aframers::af_sys::components::AComponent;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
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
	"
)]
extern "C" {
	pub fn to_init(closure: &Closure<dyn Fn(AComponent)>) -> Function;
	#[wasm_bindgen(js_name = withFirstFromThis)]
	pub fn with_component_from_this(closure: &Closure<dyn Fn(AComponent, JsValue)>) -> Function;
	#[wasm_bindgen(js_name = withFirstFromThis)]
	pub fn with_system_from_this(closure: &Closure<dyn Fn(ASystem, JsValue)>) -> Function;
}
