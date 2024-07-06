use aframers::component::core::ComponentValue;

pub trait Primitive {
	fn as_str(&self) -> &str;
}
pub struct Circle;
impl Primitive for Circle {
	fn as_str(&self) -> &str { "circle" }
}

pub type Segments = u16;

pub struct Geometry<T: Primitive> {
	primitive: Option<T>,
	segments: Option<Segments>,
}

impl<T: Primitive> Geometry<T> {
	pub fn new() -> Self {
		Self {
			primitive: None,
			segments: None,
		}
	}
}

impl Geometry<Circle> {
	pub fn set_primitive(mut self) -> Self {
		self.primitive = Some(Circle);
		self
	}
	pub fn set_segments(mut self, segments: Segments) -> Self {
		self.segments = Some(segments);
		self
	}
}

impl<T: Primitive> ComponentValue for Geometry<T> {
	fn component_name(&self) -> &str { "geometry" }

	fn component_value(&self) -> impl AsRef<str> {
		let mut clauses = vec![];
		if let Some(primitive) = &self.primitive {
			let clause = format!("primitive: {}", primitive.as_str());
			clauses.push(clause);
		}
		if let Some(segments) = self.segments {
			let clause = format!("segments: {}", segments);
			clauses.push(clause);
		}
		clauses.join("; ")
	}
}