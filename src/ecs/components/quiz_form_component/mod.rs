use std::cell::RefCell;

use aframers::af_sys::components::AComponent;
use aframers::browser::log;
use aframers::components::Position;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::js_sys::Object;

use quiz_form::QuizForm;

use crate::aframe_ex::components::core::ComponentDefinition;
use crate::aframe_ex::scene_entity_bindgen::AEntityEx;
use crate::aframe_ex::schema::SchemaProperty;
use crate::aframe_ex::schema::single_property::SinglePropertySchema;
use crate::ecs::entities::hint_entity::get_hint_cursor;
use crate::three_sys::{Color, FontLoader, Mesh, MeshBasicMaterial, Object3D, TextGeometry, TextGeometryParameters};

const COMPONENT_NAME: &'static str = "quiz-form";
const KATA_FONT_URL: &'static str = "assets/typeface/OsakaRegularMonoEnJaRestrictedReversed.json";

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
const Y_OFFSET: f32 = Y_FACTOR * OFFSET_INTENSITY * 0.5;

thread_local! {
	pub static FONT_LOADER: RefCell<FontLoader> = RefCell::new(FontLoader::new());
	pub static KATA_FONT: RefCell<Option<Object>> = RefCell::new(None);
}

fn update_entity(entity: &AEntityEx, quiz_form: &QuizForm) {
	let QuizForm { unsolved, solved, .. } = *quiz_form;
	render_indicators(unsolved, solved, entity.object3d());
}

fn render_indicators(unsolved: usize, solved: usize, object3d: Object3D) {
	let font = KATA_FONT.with_borrow(|font| font.clone());
	match font {
		None => {
			FONT_LOADER.with_borrow(|loader| {
				loader.load(KATA_FONT_URL, Closure::once_into_js(move |font: &Object| {
					KATA_FONT.set(Some(font.clone()));
					render_indicators_with_font(font, unsolved, solved, object3d);
				}).unchecked_ref());
			})
		}
		Some(font) => {
			render_indicators_with_font(&font, unsolved, solved, object3d);
		}
	}
}

fn render_indicators_with_font(font: &Object, unsolved: usize, solved: usize, object3d: Object3D) {
	match object3d.get_object_by_name("unsolved") {
		None => add_indicator("unsolved", Position(-X_OFFSET, -Y_OFFSET, 0.), unsolved, "Yellow", font),
		Some(mesh) => update_count(mesh.unchecked_into(), unsolved, font),
	}
	match object3d.get_object_by_name("solved") {
		None => add_indicator("solved", Position(X_OFFSET, -Y_OFFSET, 0.), solved, "SpringGreen", font),
		Some(mesh) => update_count(mesh.unchecked_into(), solved, font),
	}
}

fn create_material(color: &str) -> MeshBasicMaterial {
	let material = MeshBasicMaterial::new();
	material.set_color(&Color::new_str(color));
	material
}

fn update_count(mesh: Mesh, count: usize, font: &Object) {
	let geometry = create_geometry(count, font);
	mesh.set_geometry(&geometry);
}

fn add_indicator(name: &str, position: Position, count: usize, color: &str, font: &Object) {
	let mesh = Mesh::new_with_geometry_and_material(
		&create_geometry(count, font),
		&create_material(color),
	);
	mesh.set_name(name);
	mesh.position().set(position.0, position.1, position.2);
	mesh.scale().set(INDICATOR_SCALE, INDICATOR_SCALE, INDICATOR_SCALE);
	get_hint_cursor().object3d().add(&mesh);
}

fn create_geometry(count: usize, font: &Object) -> TextGeometry {
	let text = format!("{}", count);
	let geometry = TextGeometry::new(text.as_str(), create_parameters(font).as_js());
	geometry
}

fn create_parameters(font: &Object) -> TextGeometryParameters {
	let parameters = TextGeometryParameters::new();
	parameters.set_font(font);
	parameters.set_size(1.0);
	parameters.set_depth(0.05);
	parameters
}

pub mod quiz_form;

