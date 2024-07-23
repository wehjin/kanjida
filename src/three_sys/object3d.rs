use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Array;

use crate::three_sys::vector::Vector3;

#[wasm_bindgen]
extern "C" {
	#[derive(Debug)]
	pub type Object3D;
	#[wasm_bindgen(method)]
	pub fn add(this: &Object3D, object: &Object3D) -> Object3D;

	#[wasm_bindgen(method, getter)]
	pub fn children(this: &Object3D) -> Array;

	#[wasm_bindgen(method, getter)]
	pub fn name(this: &Object3D) -> String;

	#[wasm_bindgen(method, setter)]
	pub fn set_name(this: &Object3D, value: &str);

	#[wasm_bindgen(method, getter)]
	pub fn position(this: &Object3D) -> Vector3;

	#[wasm_bindgen(method, getter)]
	pub fn scale(this: &Object3D) -> Vector3;

	#[wasm_bindgen(method, js_name = getObjectByName)]
	pub fn get_object_by_name(this: &Object3D, name: &str) -> Option<Object3D>;

	#[wasm_bindgen(method, js_name = getWorldPosition)]
	pub fn get_world_position(this: &Object3D, target: &Vector3) -> Vector3;
	#[wasm_bindgen(method, js_name = localToWorld)]
	pub fn local_to_world(this: &Object3D, vector: &Vector3) -> Vector3;

	#[wasm_bindgen(method, js_name = removeFromParent)]
	pub fn remove_from_parent(this: &Object3D) -> Object3D;
}
