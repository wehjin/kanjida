use aframers::component::core::Component;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Function;

#[wasm_bindgen(
	inline_js = "\
	export function to_init(c) { return function () { c(this); }; };\
	export function partial_this(f) { return function (...args) { f(this,...args); }; };\
	"
)]
extern "C" {
	pub fn to_init(closure: &Closure<dyn Fn(Component)>) -> Function;
	pub fn partial_this(closure: &Closure<dyn Fn(Component, JsValue)>) -> Function;
}
