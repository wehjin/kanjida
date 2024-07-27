use aframers::browser::document;
use wasm_bindgen::JsCast;

use crate::aframe_ex::af_sys::{AEntityEx, ASceneEx};
use crate::aframe_ex::scenes::core::SceneEffect::Ecsv;

pub enum SceneEffect {
	Ecsv(String, String, String, String)
}

impl SceneEffect {
	pub fn ecsv(e: impl AsRef<str>, c: impl AsRef<str>, s: impl AsRef<str>, value: String) -> Self {
		Ecsv(e.as_ref().to_string(), c.as_ref().to_string(), s.as_ref().to_string(), value)
	}
}

pub fn apply_scene_effects(effects: impl AsRef<[SceneEffect]>) {
	let scene_element = document().query_selector("a-scene").unwrap().unwrap();
	let scene = scene_element.unchecked_ref::<ASceneEx>();
	for effect in effects.as_ref() {
		match effect {
			Ecsv(entity, component, setting, value) => {
				let entity_element = scene.query_selector(entity).unwrap().unwrap();
				let entity = entity_element.unchecked_ref::<AEntityEx>();
				entity.set_attribute_property(component, setting, value);
			}
		}
	}
}
