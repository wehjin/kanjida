use wasm_bindgen::prelude::wasm_bindgen;

use crate::three_sys::geometry::BufferGeometry;
use crate::three_sys::material::Material;
use crate::three_sys::object3d::Object3D;

#[wasm_bindgen(js_namespace = THREE)]
extern "C" {
	#[wasm_bindgen(extends = Object3D)]
	pub type Mesh;
	#[wasm_bindgen(constructor)]
	pub fn new() -> Mesh;
	#[wasm_bindgen(constructor)]
	pub fn new_with_geometry(geometry: &BufferGeometry) -> Mesh;
	#[wasm_bindgen(method, getter)]
	pub fn material(this: &Mesh) -> Material;
	#[wasm_bindgen(method, setter)]
	pub fn set_material(this: &Mesh, value: &Material);
	#[wasm_bindgen(method, getter)]
	pub fn geometry(this: &Mesh) -> BufferGeometry;
	#[wasm_bindgen(method, setter)]
	pub fn set_geometry(this: &Mesh, value: &BufferGeometry);
}
