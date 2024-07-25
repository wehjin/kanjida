#[derive(Debug, Copy, Clone)]
pub enum OculusTouchControlsEvent {
	AButtonDown
}

impl AsRef<str> for OculusTouchControlsEvent {
	//noinspection SpellCheckingInspection
	fn as_ref(&self) -> &str {
		match self {
			OculusTouchControlsEvent::AButtonDown => "abuttondown"
		}
	}
}