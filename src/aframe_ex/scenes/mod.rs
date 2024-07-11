use aframers::af_sys::scenes::AScene;
use aframers::components::core::ComponentValue;
use aframers::entities::Entity;
use aframers::scene::create_scene;
use wasm_bindgen::{JsCast, JsValue};

pub struct Scene(AScene);

impl Scene {
	pub fn new() -> Result<Self, JsValue> {
		let a_scene = create_scene()?.unchecked_into::<AScene>();
		Ok(Self(a_scene))
	}
	pub fn a_scene(&self) -> &AScene {
		&self.0
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