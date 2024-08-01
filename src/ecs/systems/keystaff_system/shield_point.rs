use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ShieldPoint {
	LeftBack,
	CenterBack,
	RightBack,
	LeftMiddle,
	CenterMiddle,
	RightMiddle,
	LeftFront,
	CenterFront,
	RightFront,
}

pub fn shield_points_map_glyphs(glyphs: [Option<&'static str>; 9]) -> HashMap<ShieldPoint, Option<&'static str>> {
	pub const POINTS: [ShieldPoint; 9] = [
		ShieldPoint::LeftBack, ShieldPoint::CenterBack, ShieldPoint::RightBack,
		ShieldPoint::LeftMiddle, ShieldPoint::CenterMiddle, ShieldPoint::RightMiddle,
		ShieldPoint::LeftFront, ShieldPoint::CenterFront, ShieldPoint::RightFront,
	];
	POINTS.into_iter().zip(glyphs).collect()
}