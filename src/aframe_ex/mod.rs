use std::fmt::{Display, Formatter};

use aframers::component::core::{Component, ComponentValue, register_component};
use aframers::entity::Entity;
use aframers::scene::create_scene;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Element, js_sys};
use web_sys::js_sys::{Array, Object, Reflect};

pub mod components;

#[wasm_bindgen(
	inline_js = "export function to_init(c) { return function () { c(this); }; }"
)]
extern "C" {
	pub fn to_init(closure: &Closure<dyn Fn(Component)>) -> js_sys::Function;
}

pub struct Schema(Object);
impl Schema {
	pub fn new() -> Self { Self(Object::new()) }
	pub fn push(self, name: impl AsRef<str>, field: Field) -> Self {
		Reflect::set(&self.0, &name.as_ref().into(), &field.to_object()).expect("set field");
		self
	}
	pub fn to_object(self) -> Object {
		self.0
	}
}

pub enum FieldKind {
	String,
}
impl FieldKind {
	pub fn as_str(&self) -> &str {
		match self {
			FieldKind::String => "string",
		}
	}
}

pub struct Field(JsValue, FieldKind);
impl Field {
	pub fn string(s: impl AsRef<str>) -> Self {
		Self(JsValue::from_str(s.as_ref()), FieldKind::String)
	}
	pub fn to_object(self) -> Object {
		let object = Object::new();
		Reflect::set(&object, &"default".into(), &self.0).expect("set default");
		Reflect::set(&object, &"type".into(), &self.1.as_str().into()).expect("set type");
		object
	}
}

pub struct Dependencies(Array);
impl Dependencies {
	pub fn new(component_name: impl AsRef<str>) -> Self {
		let array = Array::new_with_length(1);
		array.set(0, component_name.as_ref().into());
		Self(array)
	}
	pub fn to_array(self) -> Array { self.0 }
}

pub struct ComponentDefinition(Object);
impl ComponentDefinition {
	pub fn new() -> Self {
		ComponentDefinition(Object::new())
	}
	pub fn register(self, name: impl AsRef<str>) {
		register_component(name.as_ref(), &self.0);
	}
	pub fn set_dependencies(self, dependencies: Dependencies) -> Self {
		self.set_property("dependencies", &dependencies.to_array())
	}
	pub fn set_schema(self, schema: Schema) -> Self {
		self.set_property("schema", &schema.to_object())
	}
	pub fn set_init(self, value: impl Fn(Component) + 'static) -> Self {
		let closure = Closure::wrap(Box::new(value) as Box<dyn Fn(Component)>);
		let new_self = self.set_property("init", &to_init(&closure));
		closure.forget();
		new_self
	}
	pub fn set_property(self, name: impl AsRef<str>, value: &JsValue) -> Self {
		Reflect::set(&self.0, &name.as_ref().into(), &value).expect("set property");
		self
	}
}
#[derive(Clone, Default)]
pub struct Text {
	value: String,
	wrap_count: Option<u32>,
	align: Option<Align>,
	font: Option<String>,
	anchor: Option<Anchor>,
	baseline: Option<Baseline>,
}

impl Text {
	pub fn set_font(mut self, value: impl AsRef<str>) -> Self {
		self.font = Some(value.as_ref().to_string());
		self
	}
	pub fn set_wrap_count(mut self, value: u32) -> Self {
		self.wrap_count = Some(value);
		self
	}
	pub fn set_align(mut self, value: Align) -> Self {
		self.align = Some(value);
		self
	}
	pub fn set_anchor(mut self, value: Anchor) -> Self {
		self.anchor = Some(value);
		self
	}
	pub fn set_baseline(mut self, value: Baseline) -> Self {
		self.baseline = Some(value);
		self
	}
	pub fn new(value: impl AsRef<str>) -> Self {
		Self {
			value: value.as_ref().to_string(),
			wrap_count: None,
			align: None,
			font: None,
			anchor: None,
			baseline: None,
		}
	}
}

impl ComponentValue for Text {
	fn component_name(&self) -> &str { "text" }

	fn component_value(&self) -> impl AsRef<str> {
		let mut clauses = vec![
			format!("value: {}", self.value),
		];
		if let Some(value) = &self.font {
			clauses.push("negate: false".into());
			clauses.push(format!("font: {}", value));
		}
		if let Some(value) = self.wrap_count {
			clauses.push(format!("wrapCount: {}", value));
		}
		if let Some(value) = self.align {
			clauses.push(format!("align: {}", value));
		}
		if let Some(value) = self.anchor {
			clauses.push(format!("anchor: {}", value));
		}
		if let Some(value) = self.baseline {
			clauses.push(format!("baseline: {}", value));
		}
		clauses.join("; ")
	}
}

#[derive(Copy, Clone)]
pub enum Anchor { Left, Center, Right, Align }
impl Display for Anchor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let display = match self {
			Anchor::Left => "left",
			Anchor::Center => "center",
			Anchor::Right => "right",
			Anchor::Align => "align",
		};
		write!(f, "{}", display)
	}
}

#[derive(Copy, Clone)]
pub enum Align { Left, Center, Right }

impl Display for Align {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let display = match self {
			Align::Left => "left",
			Align::Center => "center",
			Align::Right => "right",
		};
		write!(f, "{}", display)
	}
}


#[derive(Copy, Clone)]
pub enum Baseline { Top, Center, Bottom }

impl Display for Baseline {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let display = match self {
			Baseline::Top => "top",
			Baseline::Center => "center",
			Baseline::Bottom => "bottom",
		};
		write!(f, "{}", display)
	}
}

#[derive(Copy, Clone, Default)]
pub struct RingGeometry {
	radius_inner: Option<f32>,
	radius_outer: Option<f32>,
	segments_theta: Option<u32>,
	segments_phi: Option<u32>,
	theta_start: Option<f32>,
	theta_length: Option<f32>,
}

impl RingGeometry {
	pub fn set_segments_theta(mut self, value: u32) -> Self {
		self.segments_theta = Some(value);
		self
	}
	pub fn set_radius_outer(mut self, value: f32) -> Self {
		self.radius_outer = Some(value);
		self
	}
}

impl ComponentValue for RingGeometry {
	fn component_name(&self) -> &str { "geometry" }

	fn component_value(&self) -> impl AsRef<str> {
		let mut clauses = vec!["primitive:ring".to_string()];
		if let Some(value) = self.radius_inner {
			clauses.push(format!("radiusInner: {}", value));
		}
		if let Some(value) = self.radius_outer {
			clauses.push(format!("radiusOuter: {}", value));
		}
		if let Some(value) = self.segments_theta {
			clauses.push(format!("segmentsTheta: {}", value));
		}
		if let Some(value) = self.segments_phi {
			clauses.push(format!("segmentsPhi: {}", value));
		}
		if let Some(value) = self.theta_start {
			clauses.push(format!("thetaStart: {}", value));
		}
		if let Some(value) = self.theta_length {
			clauses.push(format!("thetaLength: {}", value));
		}
		let value = clauses.join("; ");
		value
	}
}

pub struct Scene(Element);

impl Scene {
	pub fn new() -> Result<Self, JsValue> {
		let element = create_scene()?;
		let scene = Self(element);
		Ok(scene)
	}
	pub fn add_entity(self, entity: Entity) -> Result<Self, JsValue> {
		self.0.append_child(entity.element())?;
		Ok(self)
	}
	pub fn element(&self) -> &Element {
		&self.0
	}
}