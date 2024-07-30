#[derive(Debug, Copy, Clone)]
pub enum OculusTouchControlsEvent {
	GripDown,
	GripUp,
	AButtonDown,
}

impl AsRef<str> for OculusTouchControlsEvent {
	//noinspection SpellCheckingInspection
	fn as_ref(&self) -> &str {
		match self {
			Self::GripDown => "gripdown",
			Self::GripUp => "gripup",
			Self::AButtonDown => "abuttondown",
		}
	}
}
