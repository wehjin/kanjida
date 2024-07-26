use aframers::af_sys::scenes::AScene;
use aframers::components::core::ComponentAttribute;
use aframers::entities::Entity;
use aframers::scene::create_scene;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Element;

pub struct Scene(AScene);

impl Scene {
	pub fn a_scene(&self) -> &AScene {
		&self.0
	}
}

impl Scene {
	pub fn new() -> Result<Self, JsValue> {
		let a_scene = create_scene()?.unchecked_into::<AScene>();
		Ok(Self(a_scene))
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

impl From<AScene> for Scene {
	fn from(value: AScene) -> Self {
		Self(value)
	}
}