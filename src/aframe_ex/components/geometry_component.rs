use aframers::components::core::ComponentAttribute;

pub trait Primitive {
	fn as_str(&self) -> &str;
}
pub struct Circle;
impl Primitive for Circle {
	fn as_str(&self) -> &str { "circle" }
}

pub struct Box;
impl Primitive for Box {
	fn as_str(&self) -> &str { "box" }
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

impl Geometry<Box> {
	pub fn set_primitive(self) -> Self {
		Self { primitive: Some(Box), ..self }
	}
}

impl Geometry<Circle> {
	pub fn set_primitive(self) -> Self {
		Self { primitive: Some(Circle), ..self }
	}
	pub fn set_segments(self, segments: Segments) -> Self {
		Self { segments: Some(segments), ..self }
	}
}

impl<T: Primitive> ComponentAttribute for Geometry<T> {
	fn as_attribute_name(&self) -> impl AsRef<str> { "geometry" }

	fn as_attribute_str(&self) -> impl AsRef<str> {
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