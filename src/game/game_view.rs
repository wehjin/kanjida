use aframers::components::core::ComponentAttribute;

use crate::aframe_ex::scenes::core::SceneEffect;
use crate::ecs::components::hexcell_component;
use crate::ecs::entities::hint_entity::HINT_ENTITY_SELECTOR;
use crate::game::game_material::GameMaterial;
use crate::run::DETAILS_SCREEN_SELECTOR;
use crate::views::quiz_point_element_selector;

pub fn derive_scene_effects(material: &GameMaterial) -> Vec<SceneEffect> {
	let mut out = vec![
		SceneEffect::set_component_property(DETAILS_SCREEN_SELECTOR, "text", "value", &material.details),
		SceneEffect::set_component(HINT_ENTITY_SELECTOR, "value", &material.hint),
	];
	if let Some(add_state) = &material.quiz_add_selected {
		out.push(
			SceneEffect::add_state(&add_state.entity_selector, &add_state.state_name)
		);
	}
	if let Some(quiz_form) = material.quiz_form {
		out.extend(vec![
			SceneEffect::set_component(
				HINT_ENTITY_SELECTOR,
				quiz_form.as_attribute_name(),
				quiz_form.as_attribute_str(),
			),
			SceneEffect::set_component_property(
				quiz_point_element_selector(material.selected_quiz_point.unwrap()),
				hexcell_component::COMPONENT_NAME,
				hexcell_component::STATUS_SETTING,
				if quiz_form.unsolved == 0 { "1" } else { "0" },
			),
		]);
	}
	out
}
