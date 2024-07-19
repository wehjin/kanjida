use crate::ecs::components::yomikey_component::yk_settings::YkeySetting;

#[cfg(test)]
mod tests {
	use crate::ecs::components::yomikey_component::yk_state::YkeyState;
	use crate::ecs::components::yomikey_component::YkeySetting;

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

pub mod yk_settings;
pub mod yk_state;
