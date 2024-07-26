use aframers::components::core::ComponentAttribute;

pub enum RaycasterEvent {
	Intersected,
	IntersectedCleared,
}
impl RaycasterEvent {
	pub fn as_str(&self) -> &str {
		match self {
			RaycasterEvent::Intersected => "raycaster-intersected",
			RaycasterEvent::IntersectedCleared => "raycaster-intersected-cleared",
		}
	}
}

pub struct Raycaster {
	objects: Option<String>,
}

impl Raycaster {
	pub fn new() -> Self {
		Self { objects: None }
	}
	pub fn set_objects(self, value: impl AsRef<str>) -> Self {
		Self { objects: Some(value.as_ref().to_string()), ..self }
	}
}

impl ComponentAttribute for Raycaster {
	fn as_attribute_name(&self) -> impl AsRef<str> { "raycaster" }

	fn as_attribute_str(&self) -> impl AsRef<str> {
		let mut clauses = vec![];
		if let Some(value) = &self.objects {
			clauses.push(format!("objects: {}", value))
		}
		clauses.join("; ")
	}
}