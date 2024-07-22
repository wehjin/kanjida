use aframers::af_sys::components::AComponent;
use aframers::browser::log;

use quiz_form::QuizForm;

use crate::aframe_ex::components::core::ComponentDefinition;
use crate::aframe_ex::schema::SchemaProperty;
use crate::aframe_ex::schema::single_property::SinglePropertySchema;

const COMPONENT_NAME: &'static str = "quiz-form";

pub fn register_quiz_form_component() {
	let schema = SinglePropertySchema::from(QuizForm::default());
	ComponentDefinition::new()
		.set_schema(schema)
		.set_init_ref(init)
		.register(COMPONENT_NAME)
	;
}

fn init(this: &AComponent) {
	let quiz_form = QuizForm::parse_js(&this.data());
	log(&format!("INIT: {:?}", &quiz_form));
}

pub mod quiz_form;

