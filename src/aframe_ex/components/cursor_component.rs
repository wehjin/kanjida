pub enum CursorEvent {
	Click,
	Fusing,
	MouseDown,
	MouseEnter,
	MouseLeave,
	MouseUp,
}

impl CursorEvent {
	pub fn as_str(&self) -> &str {
		self.as_ref()
	}
}

impl AsRef<str> for CursorEvent {
	fn as_ref(&self) -> &str {
		match self {
			CursorEvent::Click => "click",
			CursorEvent::Fusing => "fusing",
			CursorEvent::MouseDown => "mousedown",
			CursorEvent::MouseEnter => "mouseenter",
			CursorEvent::MouseLeave => "mouseleave",
			CursorEvent::MouseUp => "mouseup"
		}
	}
}