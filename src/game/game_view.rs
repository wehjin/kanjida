use crate::aframe_ex::scenes::core::SceneEffect;
use crate::ecs::entities::hint_entity::HINT_ENTITY_SELECTOR;
use crate::game::game_material::GameMaterial;
use crate::run::DETAILS_SCREEN_SELECTOR;

pub fn game_derive_effects(material: &GameMaterial) -> Vec<SceneEffect> {
	let mut out = vec![
		SceneEffect::ecsv(DETAILS_SCREEN_SELECTOR, "text", "value", &material.details),
		SceneEffect::ecv(HINT_ENTITY_SELECTOR, "value", &material.hint),
	];
	if let Some(add_state) = &material.quiz_add_selected {
		out.push(SceneEffect::entity_add_state(&add_state.entity_selector, &add_state.state_name));
	}
	out
}
