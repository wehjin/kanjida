use std::cell::LazyCell;
use std::ops::Deref;

use aframers::af_sys::scenes::AScene;
use aframers::browser::document;
use aframers::components::core::ComponentAttribute;
use aframers::entities::Entity;
use aframers::scene::create_scene;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Element;

use crate::aframe_ex::scene_entity_bindgen::ASceneEx;

pub mod core;

thread_local! {
	 pub static A_SCENE: LazyCell<ASceneEx> = LazyCell::new(|| {
		let scene = create_scene().unwrap();
		document().body().unwrap().append_child(&scene).unwrap();
		document().query_selector("a-scene").unwrap().unwrap().unchecked_into::<ASceneEx>()
	})
}

pub struct Scene(ASceneEx);

impl Scene {
	pub fn a_scene(&self) -> &ASceneEx {
		&self.0
	}
}

impl Scene {
	pub fn get() -> Result<Self, JsValue> {
		let scene_element = A_SCENE.with(|it| it.deref().clone());
		Ok(Self(scene_element))
	}
	pub fn set_component_attribute(self, value: impl ComponentAttribute) -> Result<Self, JsValue> {
		self.0.set_attribute(value.as_attribute_name().as_ref(), value.as_attribute_str().as_ref())?;
		Ok(self)
	}
	pub fn add_entity(self, entity: Entity) -> Result<Self, JsValue> {
		self.0.append_child(entity.a_entity())?;
		Ok(self)
	}
	pub fn add_assets(self, assets: Element) -> Self {
		self.0.append_child(&assets).unwrap();
		self
	}
	pub fn update_element(self, f: impl Fn(&AScene)) -> Self {
		f(self.a_scene());
		self
	}
}

impl From<ASceneEx> for Scene {
	fn from(value: ASceneEx) -> Self {
		Self(value)
	}
}