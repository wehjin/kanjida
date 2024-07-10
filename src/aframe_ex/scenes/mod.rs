use aframers::af_sys::scenes::AScene;
use aframers::components::core::ComponentValue;
use aframers::entities::Entity;
use aframers::scene::create_scene;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::Reflect;

use crate::aframe_ex::systems::ASystem;

pub struct Scene(AScene);

impl Scene {
	pub fn new() -> Result<Self, JsValue> {
		let a_scene = create_scene()?.unchecked_into::<AScene>();
		Ok(Self(a_scene))
	}
	pub fn a_scene(&self) -> &AScene {
		&self.0
	}
	pub fn a_system(&self, name: impl AsRef<str>) -> ASystem {
		let systems = self.0.systems();
		let system = Reflect::get(&systems, &name.as_ref().into()).expect("system");
		system.unchecked_into()
	}
	pub fn set_component(self, component: impl ComponentValue) -> Result<Self, JsValue> {
		self.0.set_attribute(component.component_name(), component.component_value().as_ref())?;
		Ok(self)
	}
	pub fn add_entity(self, entity: Entity) -> Result<Self, JsValue> {
		self.0.append_child(entity.a_entity())?;
		Ok(self)
	}
}

impl From<AScene> for Scene {
	fn from(value: AScene) -> Self {
		Self(value)
	}
}