use wasm_bindgen::JsCast;

use crate::aframe_ex::af_sys::{AEntityEx, ASceneEx};
use crate::aframe_ex::scenes::core::SceneEffect::{Ecsv, Ecv};

pub enum SceneEffect {
	Ecsv(String, String, String, String),
	Ecv(String, String, String),
}

impl SceneEffect {
	pub fn ecsv(e: impl AsRef<str>, c: impl AsRef<str>, s: impl AsRef<str>, value: impl AsRef<str>) -> Self {
		Ecsv(e.as_ref().to_string(), c.as_ref().to_string(), s.as_ref().to_string(), value.as_ref().to_string())
	}
	pub fn ecv(e: impl AsRef<str>, c: impl AsRef<str>, value: impl AsRef<str>) -> Self {
		Ecv(e.as_ref().to_string(), c.as_ref().to_string(), value.as_ref().to_string())
	}
}

impl SceneEffect {
	pub fn apply_in_scene(&self, scene: &ASceneEx) {
		match self {
			Ecsv(entity, component, setting, value) => {
				let entity_element = scene.query_selector(entity).unwrap().unwrap();
				let entity = entity_element.unchecked_ref::<AEntityEx>();
				entity.set_attribute_property(component, setting, value);
			}
			Ecv(entity, component, value) => {
				let entity_element = scene.query_selector(entity).unwrap().unwrap();
				let entity = entity_element.unchecked_ref::<AEntityEx>();
				entity.set_attribute(component, value).unwrap();
			}
		}
	}
}

pub fn scene_apply_effects(scene: &ASceneEx, effects: impl AsRef<[SceneEffect]>) {
	for effect in effects.as_ref() {
		effect.apply_in_scene(scene)
	}
}