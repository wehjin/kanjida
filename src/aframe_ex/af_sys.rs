use aframers::af_sys::entities::AEntity;
use aframers::af_sys::scenes::AScene;
use aframers::components::Position;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::three_sys::object3d::Object3D;
use crate::three_sys::vector::Vector3;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AEntity)]
	pub type AEntityEx;
	#[wasm_bindgen(method, getter, js_name = object3D)]
	pub fn object3d(this: &AEntityEx) -> Object3D;
	#[wasm_bindgen(method, js_name = getObject3D)]
	pub fn get_object3d_kind(this: &AEntityEx, kind: &str) -> Object3D;
	#[wasm_bindgen(method, js_name = setObject3D)]
	pub fn set_object3d_kind(this: &AEntityEx, kind: &str, object: &Object3D);
}

impl AEntityEx {
	pub fn local_position_to_world(&self, position: Position) -> Position {
		let local = Vector3::new(position.0, position.1, position.2);
		let world = self.object3d().local_to_world(&local);
		Position(world.x(), world.y(), world.z())
	}
	pub fn local_position(&self) -> Position {
		let vector = self.object3d().position();
		Position(vector.x(), vector.y(), vector.z())
	}
	pub fn world_position_in_new_vector(&self) -> Vector3 {
		let cell_vector = Vector3::origin();
		self.object3d().get_world_position(&cell_vector);
		cell_vector
	}
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AScene)]
	pub type ASceneEx;

	#[wasm_bindgen(method, getter)]
	pub fn yomigun_target_position(this: &ASceneEx) -> Option<Vector3>;
	#[wasm_bindgen(method, setter, structural)]
	pub fn set_yomigun_target_position(this: &ASceneEx, value: Option<Vector3>);
}
