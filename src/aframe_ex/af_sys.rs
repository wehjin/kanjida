use aframers::af_sys::entities::AEntity;
use aframers::components::Position;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::three_sys::{Object3D, Vector3};

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AEntity)]
	pub type AEntityEx;
	#[wasm_bindgen(method, getter)]
	pub fn object3D(this: &AEntityEx) -> Object3D;
}

impl AEntityEx {
	pub fn local_position_to_world(&self, position: Position) -> Position {
		let local = Vector3::new(position.0, position.1, position.2);
		let world = self.object3D().local_to_world(&local);
		Position(world.x(), world.y(), world.z())
	}
}

