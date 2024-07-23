use aframers::af_sys::components::AComponent;
use aframers::browser::log;
use wasm_bindgen::JsCast;

use quiz_form::QuizForm;

use crate::aframe_ex::af_sys::AEntityEx;
use crate::aframe_ex::components::core::ComponentDefinition;
use crate::aframe_ex::schema::SchemaProperty;
use crate::aframe_ex::schema::single_property::SinglePropertySchema;
use crate::three_sys;
use crate::three_sys::Color;

const COMPONENT_NAME: &'static str = "quiz-form";

pub fn register_quiz_form_component() {
	let schema = SinglePropertySchema::from(QuizForm::default());
	ComponentDefinition::new()
		.set_schema(schema)
		.set_update_ref(update)
		.register(COMPONENT_NAME)
	;
}

fn update(this: &AComponent) {
	let quiz_form = QuizForm::parse_js(&this.data());
	log(&format!("UPDATE: {:?}", &quiz_form));
	update_entity(this.a_entity().unchecked_ref(), &quiz_form);
}


const INDICATOR_SCALE: f32 = 0.2;
const OFFSET_INTENSITY: f32 = 0.25;
const X_FACTOR: f32 = 1.5;
const Y_FACTOR: f32 = 1.732;
const X_OFFSET: f32 = X_FACTOR * OFFSET_INTENSITY;
const Y_OFFSET: f32 = Y_FACTOR * OFFSET_INTENSITY;

fn update_entity(entity: &AEntityEx, quiz_form: &QuizForm) {
	let object3d = entity.object3d();
	let unsolved_mesh = object3d.get_object_by_name("unsolved");
	match (quiz_form.unsolved > 0, unsolved_mesh) {
		(true, None) => {
			let material = three_sys::MeshBasicMaterial::new();
			material.set_color(&Color::new_str("Yellow"));
			let mesh = three_sys::Mesh::new_with_geometry_and_material(
				&three_sys::BoxGeometry::new(),
				&material,
			);
			mesh.set_name("unsolved");
			mesh.position().set(-X_OFFSET, -Y_OFFSET, 0.);
			mesh.scale().set(INDICATOR_SCALE, INDICATOR_SCALE, INDICATOR_SCALE);
			object3d.add(&mesh);
		}
		(true, Some(_)) => (),
		(false, None) => (),
		(false, Some(mesh)) => {
			mesh.remove_from_parent();
		}
	}
}

pub mod quiz_form;

