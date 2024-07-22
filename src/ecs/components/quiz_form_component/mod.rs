use aframers::af_sys::components::AComponent;
use aframers::browser::log;

use attribute::QuizFormAttribute;

use crate::aframe_ex::components::core::ComponentDefinition;
use crate::aframe_ex::js::log_value;
use crate::aframe_ex::schema::single_property::SinglePropertySchema;

const COMPONENT_NAME: &'static str = "quiz-form";

pub fn register_quiz_form_component() {
	let schema = SinglePropertySchema::from(QuizFormAttribute::default());
	ComponentDefinition::new()
		.set_schema(schema)
		.set_init_ref(init)
		.register(COMPONENT_NAME)
	;
}

fn init(this: &AComponent) {
	log(&format!("INIT {}", COMPONENT_NAME));
	log_value(&this.data());
}

pub mod attribute;

