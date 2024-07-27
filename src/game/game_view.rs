use crate::aframe_ex::scenes::core::SceneEffect;
use crate::ecs::entities::hint_entity::HINT_ENTITY_SELECTOR;
use crate::game::game_material::GameMaterial;
use crate::run::DETAILS_SCREEN_SELECTOR;

pub fn game_derive_effects(material: &GameMaterial) -> Vec<SceneEffect> {
	vec![
		SceneEffect::ecsv(DETAILS_SCREEN_SELECTOR, "text", "value", &material.details),
		SceneEffect::ecv(HINT_ENTITY_SELECTOR, "value", &material.hint),
	]
}
