use web_sys::CustomEvent;

use StateEventKind::StateRemoved;

use crate::aframe_ex::events::StateEventKind::StateAdded;

pub mod core;

pub enum StateEventKind {
	StateAdded,
	StateRemoved,
}

impl AsRef<str> for StateEventKind {
	//noinspection SpellCheckingInspection
	fn as_ref(&self) -> &str {
		match self {
			StateAdded => "stateadded",
			StateRemoved => "stateremoved",
		}
	}
}

pub struct StateEvent(String, StateEventKind);
impl StateEvent {
	pub fn state(&self) -> &str { &self.0 }
	pub fn kind(&self) -> &StateEventKind { &self.1 }

	pub fn try_added(custom: &CustomEvent) -> Option<Self> { Self::try_from_custom(custom, StateAdded) }
	pub fn try_removed(custom: &CustomEvent) -> Option<Self> { Self::try_from_custom(custom, StateRemoved) }

	pub fn try_from_custom(custom: &CustomEvent, kind: StateEventKind) -> Option<Self> {
		if custom.type_() == kind.as_ref() {
			let detail = custom.detail().as_string().expect("string detail");
			Some(Self(detail, kind))
		} else {
			None
		}
	}
}
