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
	UnderLeftBack,
	UnderCenterBack,
	UnderRightBack,
	UnderLeftMiddle,
	UnderCenterMiddle,
	UnderRightMiddle,
	UnderLeftFront,
	UnderCenterFront,
	UnderRightFront,
}

impl ShieldPoint {
	pub fn to_row_col(&self) -> RowCol {
		match self {
			Self::OverLeftBack => RowCol::LeftBack,
			Self::OverCenterBack => RowCol::CenterBack,
			Self::OverRightBack => RowCol::RightBack,
			Self::OverLeftMiddle => RowCol::LeftMiddle,
			Self::OverCenterMiddle => RowCol::CenterMiddle,
			Self::OverRightMiddle => RowCol::RightMiddle,
			Self::OverLeftFront => RowCol::LeftFront,
			Self::OverCenterFront => RowCol::CenterFront,
			Self::OverRightFront => RowCol::RightFront,
			Self::GroundLeftBack => RowCol::LeftBack,
			Self::GroundCenterBack => RowCol::CenterBack,
			Self::GroundRightBack => RowCol::RightBack,
			Self::GroundLeftMiddle => RowCol::LeftMiddle,
			Self::GroundCenterMiddle => RowCol::CenterMiddle,
			Self::GroundRightMiddle => RowCol::RightMiddle,
			Self::GroundLeftFront => RowCol::LeftFront,
			Self::GroundCenterFront => RowCol::CenterFront,
			Self::GroundRightFront => RowCol::RightFront,
			Self::UnderLeftBack => RowCol::LeftBack,
			Self::UnderCenterBack => RowCol::CenterBack,
			Self::UnderRightBack => RowCol::RightBack,
			Self::UnderLeftMiddle => RowCol::LeftMiddle,
			Self::UnderCenterMiddle => RowCol::CenterMiddle,
			Self::UnderRightMiddle => RowCol::RightMiddle,
			Self::UnderLeftFront => RowCol::LeftFront,
			Self::UnderCenterFront => RowCol::CenterFront,
			Self::UnderRightFront => RowCol::RightFront
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Floor { Over, Ground, Under }


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
			Floor::Over => self.to_point_on_over_floor(),
			Floor::Ground => self.to_point_on_ground_floor(),
			Floor::Under => self.to_point_on_under_floor(),
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
	pub fn to_point_on_under_floor(&self) -> ShieldPoint {
		match self {
			Self::LeftBack => ShieldPoint::UnderLeftBack,
			Self::CenterBack => ShieldPoint::UnderCenterBack,
			Self::RightBack => ShieldPoint::UnderRightBack,
			Self::LeftMiddle => ShieldPoint::UnderLeftMiddle,
			Self::CenterMiddle => ShieldPoint::UnderCenterMiddle,
			Self::RightMiddle => ShieldPoint::UnderRightMiddle,
			Self::LeftFront => ShieldPoint::UnderLeftFront,
			Self::CenterFront => ShieldPoint::UnderCenterFront,
			Self::RightFront => ShieldPoint::UnderRightFront,
		}
	}
}

pub fn shield_points_map_glyphs_ignore_floor(glyphs: [Option<&'static str>; 9]) -> HashMap<ShieldPoint, Option<&'static str>> {
	let mut map = (&ALL_POINTS[0..9]).iter().cloned().zip(glyphs).collect::<HashMap<_, _>>();
	map.extend((&ALL_POINTS[9..18]).iter().cloned().zip(glyphs).collect::<HashMap<_, _>>());
	map.extend((&ALL_POINTS[18..27]).iter().cloned().zip(glyphs).collect::<HashMap<_, _>>());
	map
}

pub fn shield_points_map_glyphs_clamp_under(glyphs: [Option<&'static str>; 18]) -> HashMap<ShieldPoint, Option<&'static str>> {
	let over_and_ground = ALL_POINTS.as_slice()[0..18].iter().cloned().zip(glyphs).collect::<HashMap<_, _>>();
	let under_glyphs = glyphs.as_slice()[9..18].to_vec();
	let under = ALL_POINTS.as_slice()[18..27].iter().cloned().zip(under_glyphs).collect::<HashMap<_, _>>();
	let mut map = over_and_ground;
	map.extend(under);
	map
}

pub fn shield_points_map_glyphs(glyphs: [Option<&'static str>; 27]) -> HashMap<ShieldPoint, Option<&'static str>> {
	ALL_POINTS.into_iter().zip(glyphs).collect::<HashMap<_, _>>()
}

const ALL_POINTS: [ShieldPoint; 27] = [
	ShieldPoint::OverLeftBack, ShieldPoint::OverCenterBack, ShieldPoint::OverRightBack,
	ShieldPoint::OverLeftMiddle, ShieldPoint::OverCenterMiddle, ShieldPoint::OverRightMiddle,
	ShieldPoint::OverLeftFront, ShieldPoint::OverCenterFront, ShieldPoint::OverRightFront,
	ShieldPoint::GroundLeftBack, ShieldPoint::GroundCenterBack, ShieldPoint::GroundRightBack,
	ShieldPoint::GroundLeftMiddle, ShieldPoint::GroundCenterMiddle, ShieldPoint::GroundRightMiddle,
	ShieldPoint::GroundLeftFront, ShieldPoint::GroundCenterFront, ShieldPoint::GroundRightFront,
	ShieldPoint::UnderLeftBack, ShieldPoint::UnderCenterBack, ShieldPoint::UnderRightBack,
	ShieldPoint::UnderLeftMiddle, ShieldPoint::UnderCenterMiddle, ShieldPoint::UnderRightMiddle,
	ShieldPoint::UnderLeftFront, ShieldPoint::UnderCenterFront, ShieldPoint::UnderRightFront,
];
