use crate::aframe_ex::scenes::core::SceneEffect;
use crate::game::game_material::GameMaterial;
use crate::run::DETAILS_SCREEN_SELECTOR;


pub fn derive_game_effects(material: &GameMaterial) -> Vec<SceneEffect> {
	let details = SceneEffect::ecsv(DETAILS_SCREEN_SELECTOR, "text", "value", material.details.to_owned());
	vec![details]
}
