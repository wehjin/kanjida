use wasm_bindgen::JsCast;

use crate::aframe_ex::af_sys::{AEntityEx, ASceneEx};
use crate::aframe_ex::scenes::core::SceneEffect::{EntityAddState, SetComponent, SetComponentProperty};

pub enum SceneEffect {
	SetComponentProperty(String, String, String, String),
	SetComponent(String, String, String),
	EntityAddState(String, String),
}

impl SceneEffect {
	pub fn set_component_property(entity: impl AsRef<str>, component: impl AsRef<str>, setting: impl AsRef<str>, value: impl AsRef<str>) -> Self {
		SetComponentProperty(entity.as_ref().to_string(), component.as_ref().to_string(), setting.as_ref().to_string(), value.as_ref().to_string())
	}
	pub fn set_component(entity: impl AsRef<str>, component: impl AsRef<str>, value: impl AsRef<str>) -> Self {
		SetComponent(entity.as_ref().to_string(), component.as_ref().to_string(), value.as_ref().to_string())
	}
	pub fn add_state(entity: impl AsRef<str>, state: impl AsRef<str>) -> Self {
		EntityAddState(entity.as_ref().to_string(), state.as_ref().to_string())
	}
}

impl SceneEffect {
	pub fn apply_in_scene(&self, scene: &ASceneEx) {
		match self {
			SetComponentProperty(entity, component, setting, value) => {
				let entity_element = scene.query_selector(entity).unwrap()
					.unwrap_or_else(|| panic!("No element for selector '{}'", entity));
				let entity = entity_element.unchecked_ref::<AEntityEx>();
				entity.set_attribute_property(component, setting, value);
			}
			SetComponent(entity, component, value) => {
				let entity_element = scene.query_selector(entity).unwrap().unwrap();
				let entity = entity_element.unchecked_ref::<AEntityEx>();
				entity.set_attribute(component, value).unwrap();
			}
			EntityAddState(entity, state) => {
				let entity_element = scene.query_selector(entity).unwrap().unwrap();
				let entity = entity_element.unchecked_ref::<AEntityEx>();
				if !entity.is_state(state) {
					entity.add_state(state);
				}
			}
		}
	}
}

pub fn scene_apply_effects(scene: &ASceneEx, effects: impl AsRef<[SceneEffect]>) {
	for effect in effects.as_ref() {
		effect.apply_in_scene(scene)
	}
}