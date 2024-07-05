pub const RAYCASTER: &str = "raycaster";

pub enum RayCasterEvent {
	Intersected,
	IntersectedCleared,
}
impl RayCasterEvent {
	pub fn as_str(&self) -> &str {
		match self {
			RayCasterEvent::Intersected => "raycaster-intersected",
			RayCasterEvent::IntersectedCleared => "raycaster-intersected-cleared",
		}
	}
}
