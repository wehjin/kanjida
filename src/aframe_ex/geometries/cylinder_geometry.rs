use aframers::components::core::{ComponentAttribute, ComponentSetting};

use crate::aframe_ex::components::core::component_settings_as_string;

pub struct CylinderGeometry(pub Vec<CylinderGeometrySetting>);
impl ComponentAttribute for CylinderGeometry {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		"geometry"
	}
	fn as_attribute_str(&self) -> impl AsRef<str> {
		component_settings_as_string(&self.0)
	}
}

pub enum CylinderGeometrySetting {
	Primitive,
	/// Radius of the cylinder.
	Radius(f32),
	/// Height of the cylinder.
	Height(f32),
	/// Number of segmented faces around the circumference of the cylinder.
	SegmentsRadial(usize),
	/// Number of rows of faces along the height of the cylinder.
	SegmentsHeight(usize),
	/// Whether the ends of the cylinder are open (true) or capped (false).
	OpenEnded(bool),
	/// Whether the ends of the cylinder are open (true) or capped (false).
	ThetaStart(f32),
	/// Central angle in degrees.
	ThetaLength(f32),
}

impl ComponentSetting for CylinderGeometrySetting {
	fn as_setting_name(&self) -> impl AsRef<str> {
		match self {
			CylinderGeometrySetting::Primitive => "primitive",
			CylinderGeometrySetting::Radius(_) => "radius",
			CylinderGeometrySetting::Height(_) => "height",
			CylinderGeometrySetting::SegmentsRadial(_) => "segmentsRadial",
			CylinderGeometrySetting::SegmentsHeight(_) => "segmentsHeight",
			CylinderGeometrySetting::OpenEnded(_) => "openEnded",
			CylinderGeometrySetting::ThetaStart(_) => "thetaStart",
			CylinderGeometrySetting::ThetaLength(_) => "thetaLength",
		}
	}

	fn as_setting_str(&self) -> impl AsRef<str> {
		match self {
			CylinderGeometrySetting::Primitive => "cylinder".to_string(),
			CylinderGeometrySetting::Radius(value) => format!("{}", value),
			CylinderGeometrySetting::Height(value) => format!("{}", value),
			CylinderGeometrySetting::SegmentsRadial(value) => format!("{}", value),
			CylinderGeometrySetting::SegmentsHeight(value) => format!("{}", value),
			CylinderGeometrySetting::OpenEnded(value) => format!("{}", value),
			CylinderGeometrySetting::ThetaStart(value) => format!("{}", value),
			CylinderGeometrySetting::ThetaLength(value) => format!("{}", value),
		}
	}
}
