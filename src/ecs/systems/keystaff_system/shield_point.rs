use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ShieldPoint {
	OverLeftBack,
	OverCenterBack,
	OverRightBack,
	OverLeftMiddle,
	OverCenterMiddle,
	OverRightMiddle,
	OverLeftFront,
	OverCenterFront,
	OverRightFront,
	GroundLeftBack,
	GroundCenterBack,
	GroundRightBack,
	GroundLeftMiddle,
	GroundCenterMiddle,
	GroundRightMiddle,
	GroundLeftFront,
	GroundCenterFront,
	GroundRightFront,
}

impl ShieldPoint {
	pub fn to_row_col(&self) -> RowCol {
		match self {
			ShieldPoint::OverLeftBack => RowCol::LeftBack,
			ShieldPoint::OverCenterBack => RowCol::CenterBack,
			ShieldPoint::OverRightBack => RowCol::RightBack,
			ShieldPoint::OverLeftMiddle => RowCol::LeftMiddle,
			ShieldPoint::OverCenterMiddle => RowCol::CenterMiddle,
			ShieldPoint::OverRightMiddle => RowCol::RightMiddle,
			ShieldPoint::OverLeftFront => RowCol::LeftFront,
			ShieldPoint::OverCenterFront => RowCol::CenterFront,
			ShieldPoint::OverRightFront => RowCol::RightFront,
			ShieldPoint::GroundLeftBack => RowCol::LeftBack,
			ShieldPoint::GroundCenterBack => RowCol::CenterBack,
			ShieldPoint::GroundRightBack => RowCol::RightBack,
			ShieldPoint::GroundLeftMiddle => RowCol::LeftMiddle,
			ShieldPoint::GroundCenterMiddle => RowCol::CenterMiddle,
			ShieldPoint::GroundRightMiddle => RowCol::RightMiddle,
			ShieldPoint::GroundLeftFront => RowCol::LeftFront,
			ShieldPoint::GroundCenterFront => RowCol::CenterFront,
			ShieldPoint::GroundRightFront => RowCol::RightFront
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Floor { Ground, Over }


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RowCol {
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

impl RowCol {
	pub fn to_point_on_floor(&self, floor: Floor) -> ShieldPoint {
		match floor {
			Floor::Ground => self.to_point_on_ground_floor(),
			Floor::Over => self.to_point_on_over_floor()
		}
	}
	//noinspection DuplicatedCode
	pub fn to_point_on_ground_floor(&self) -> ShieldPoint {
		match self {
			Self::LeftBack => ShieldPoint::GroundLeftBack,
			Self::CenterBack => ShieldPoint::GroundCenterBack,
			Self::RightBack => ShieldPoint::GroundRightBack,
			Self::LeftMiddle => ShieldPoint::GroundLeftMiddle,
			Self::CenterMiddle => ShieldPoint::GroundCenterMiddle,
			Self::RightMiddle => ShieldPoint::GroundRightMiddle,
			Self::LeftFront => ShieldPoint::GroundLeftFront,
			Self::CenterFront => ShieldPoint::GroundCenterFront,
			Self::RightFront => ShieldPoint::GroundRightFront,
		}
	}
	//noinspection DuplicatedCode
	pub fn to_point_on_over_floor(&self) -> ShieldPoint {
		match self {
			Self::LeftBack => ShieldPoint::OverLeftBack,
			Self::CenterBack => ShieldPoint::OverCenterBack,
			Self::RightBack => ShieldPoint::OverRightBack,
			Self::LeftMiddle => ShieldPoint::OverLeftMiddle,
			Self::CenterMiddle => ShieldPoint::OverCenterMiddle,
			Self::RightMiddle => ShieldPoint::OverRightMiddle,
			Self::LeftFront => ShieldPoint::OverLeftFront,
			Self::CenterFront => ShieldPoint::OverCenterFront,
			Self::RightFront => ShieldPoint::OverRightFront,
		}
	}
}

pub fn shield_points_map_glyphs_ignore_floor(glyphs: [Option<&'static str>; 9]) -> HashMap<ShieldPoint, Option<&'static str>> {
	let mut map = (&ALL_POINTS[0..9]).iter().cloned().zip(glyphs).collect::<HashMap<_, _>>();
	map.extend((&ALL_POINTS[9..18]).iter().cloned().zip(glyphs).collect::<HashMap<_, _>>());
	map
}

pub fn shield_points_map_glyphs(glyphs: [Option<&'static str>; 18]) -> HashMap<ShieldPoint, Option<&'static str>> {
	ALL_POINTS.into_iter().zip(glyphs).collect::<HashMap<_, _>>()
}

const ALL_POINTS: [ShieldPoint; 18] = [
	ShieldPoint::OverLeftBack, ShieldPoint::OverCenterBack, ShieldPoint::OverRightBack,
	ShieldPoint::OverLeftMiddle, ShieldPoint::OverCenterMiddle, ShieldPoint::OverRightMiddle,
	ShieldPoint::OverLeftFront, ShieldPoint::OverCenterFront, ShieldPoint::OverRightFront,
	ShieldPoint::GroundLeftBack, ShieldPoint::GroundCenterBack, ShieldPoint::GroundRightBack,
	ShieldPoint::GroundLeftMiddle, ShieldPoint::GroundCenterMiddle, ShieldPoint::GroundRightMiddle,
	ShieldPoint::GroundLeftFront, ShieldPoint::GroundCenterFront, ShieldPoint::GroundRightFront,
];
