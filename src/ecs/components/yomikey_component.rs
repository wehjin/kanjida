use crate::ecs::components::yomikey_component::settings::YkeySetting;

#[cfg(test)]
mod tests {
	use crate::ecs::components::yomikey_component::{YkeySetting, YkeyState};

	#[test]
	fn state_works() {
		let settings = [YkeySetting::Glyph("M".into()), YkeySetting::Uv(1, 2)];
		let state = YkeyState::init(settings);
		assert_eq!(state.glyph(), "M");
		assert_eq!(state.uv(), (1, 2));
		assert_eq!(state.is_focused(), false);
		assert_eq!(state.clicks(), 0);
		let state = state.enter().click();
		assert_eq!(state.is_focused(), true);
		assert_eq!(state.clicks(), 1);
		let state = state.leave();
		assert_eq!(state.is_focused(), false);
	}
}

#[derive(Debug, Clone, Default)]
pub struct YkeyState {
	age: usize,
	uv: (usize, usize),
	glyph: String,
	enters: usize,
	leaves: usize,
	clicks: usize,
}
pub mod queries {
	use crate::ecs::components::yomikey_component::YkeyState;

	impl YkeyState {
		pub fn glyph(&self) -> &str { &self.glyph }
		pub fn uv(&self) -> (usize, usize) { self.uv }
		pub fn is_focused(&self) -> bool { self.enters > self.leaves }
		pub fn clicks(&self) -> usize { self.clicks }
	}
}

pub mod actions {
	use crate::ecs::components::yomikey_component::settings::YkeySetting;
	use crate::ecs::components::yomikey_component::YkeyState;

	impl YkeyState {
		pub fn init(settings: impl AsRef<[YkeySetting]>) -> Self {
			let (glyph, uv) = unwrap_settings(settings);
			Self { uv, glyph, ..Self::default() }
		}
		pub fn enter(self) -> Self {
			if self.enters <= self.leaves {
				Self { age: self.age + 1, enters: self.leaves + 1, ..self }
			} else {
				self
			}
		}
		pub fn leave(self) -> Self {
			if self.leaves < self.enters {
				Self { age: self.age, leaves: self.enters, ..self }
			} else {
				self
			}
		}
		pub fn click(self) -> Self {
			Self { age: self.age + 1, clicks: self.clicks + 1, ..self }
		}
	}
	fn unwrap_settings(settings: impl AsRef<[YkeySetting]>) -> (String, (usize, usize)) {
		let mut glyph: Option<String> = None;
		let mut uv: Option<(usize, usize)> = None;
		for setting in settings.as_ref() {
			match setting {
				YkeySetting::Glyph(value) => glyph = Some(value.to_string()),
				YkeySetting::Uv(u, v) => uv = Some((*u, *v)),
			}
		}
		let glyph = glyph.unwrap_or_else(|| "X".into());
		let uv = uv.unwrap_or((0, 0));
		(glyph, uv)
	}
}
pub mod settings {
	#[derive(Debug, Clone)]
	pub enum YkeySetting {
		Glyph(String),
		Uv(usize, usize),
	}
}
