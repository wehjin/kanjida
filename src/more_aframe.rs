use aframers::component::core::ComponentValue;
use aframers::entity::Entity;
use aframers::scene::create_scene;
use wasm_bindgen::JsValue;
use web_sys::Element;

#[derive(Copy, Clone, Default)]
pub struct RingGeometry {
	radius_inner: Option<f32>,
	radius_outer: Option<f32>,
	segments_theta: Option<u32>,
	segments_phi: Option<u32>,
	theta_start: Option<f32>,
	theta_length: Option<f32>,
}

impl RingGeometry {
	pub fn set_segments_theta(mut self, value: u32) -> Self {
		self.segments_theta = Some(value);
		self
	}
}

impl ComponentValue for RingGeometry {
	fn component_name(&self) -> &str { "geometry" }

	fn component_value(&self) -> impl AsRef<str> {
		let mut clauses = vec!["primitive:ring".to_string()];
		if let Some(value) = self.radius_inner {
			clauses.push(format!("radiusInner: {}", value));
		}
		if let Some(value) = self.radius_outer {
			clauses.push(format!("radiusOuter: {}", value));
		}
		if let Some(value) = self.segments_theta {
			clauses.push(format!("segmentsTheta: {}", value));
		}
		if let Some(value) = self.segments_phi {
			clauses.push(format!("segmentsPhi: {}", value));
		}
		if let Some(value) = self.theta_start {
			clauses.push(format!("thetaStart: {}", value));
		}
		if let Some(value) = self.theta_length {
			clauses.push(format!("thetaLength: {}", value));
		}
		let value = clauses.join("; ");
		value
	}
}

pub struct Scene(Element);

impl Scene {
	pub fn new() -> Result<Self, JsValue> {
		let element = create_scene()?;
		let scene = Self(element);
		Ok(scene)
	}
	pub fn add_entity(self, entity: Entity) -> Result<Self, JsValue> {
		self.0.append_child(entity.element())?;
		Ok(self)
	}
	pub fn element(&self) -> &Element {
		&self.0
	}
}