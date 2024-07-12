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

pub enum CursorState {
	CursorHovered,
	CursorFusing,
	CursorHovering,
}

impl AsRef<str> for CursorState {
	fn as_ref(&self) -> &str {
		match self {
			CursorState::CursorHovered => "cursor-hovered",
			CursorState::CursorFusing => "cursor-fusing",
			CursorState::CursorHovering => "cursor-hovering",
		}
	}
}