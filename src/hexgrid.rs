use std::ops::{Add, Mul};

const HEX_SIZE: f32 = 1.0;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum HexDirection {
	NorthEast,
	SouthEast,
	South,
	SouthWest,
	NorthWest,
	North,
}

impl HexDirection {
	pub const fn to_vector(&self) -> HexCoord {
		match self {
			HexDirection::South => HexCoord::new(0, 1),
			HexDirection::SouthWest => HexCoord::new(-1, 1),
			HexDirection::NorthWest => HexCoord::new(-1, 0),
			HexDirection::North => HexCoord::new(0, -1),
			HexDirection::NorthEast => HexCoord::new(1, -1),
			HexDirection::SouthEast => HexCoord::new(1, 0),
		}
	}
}


#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct HexCoord {
	pub q: i32,
	pub r: i32,
}

impl HexCoord {
	pub const ORIGIN: Self = HexCoord::new(0, 0);
	pub const fn new(q: i32, r: i32) -> Self { Self { q, r } }
	pub fn s(&self) -> i32 { -self.q - self.r }
	pub fn to_pixel(&self) -> PixelCoord {
		let q = self.q as f32;
		let r = self.r as f32;
		let x = HEX_SIZE * (3. / 2. * q);
		let y = HEX_SIZE * (f32::sqrt(3.) / 2. * q + f32::sqrt(3.) * r);
		return PixelCoord { x, y };
	}
	pub fn to_ring(&self, radius: usize) -> Vec<Self> {
		const MOVEMENTS: [HexDirection; 6] = [
			HexDirection::South,
			HexDirection::SouthWest,
			HexDirection::NorthWest,
			HexDirection::North,
			HexDirection::NorthEast,
			HexDirection::SouthEast,
		];
		let mut results = vec![];
		let mut hex = *self + (MOVEMENTS[4].to_vector() * radius as i32);
		for travel in &MOVEMENTS[0..6] {
			let travel_vector = travel.to_vector();
			for _ in 0..radius {
				results.push(hex);
				hex = hex + travel_vector;
			}
		}
		results
	}
	pub fn to_spiral(&self, outer_radius: usize) -> Vec<Self> {
		let mut results = vec![*self];
		for radius in 1..=outer_radius {
			let ring = self.to_ring(radius);
			results.extend(ring);
		}
		results
	}

	pub fn iter_spiral(&self) -> impl Iterator<Item=HexCoord> {
		HexSpiral::new(*self)
	}
}
impl Add<HexCoord> for HexCoord {
	type Output = Self;

	fn add(self, rhs: HexCoord) -> Self::Output {
		let q = self.q + rhs.q;
		let r = self.r + rhs.r;
		Self { q, r }
	}
}

impl Mul<i32> for HexCoord {
	type Output = Self;

	fn mul(self, rhs: i32) -> Self::Output {
		Self { q: self.q * rhs, r: self.r * rhs }
	}
}

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct PixelCoord {
	pub x: f32,
	pub y: f32,
}

impl PixelCoord {
	pub fn flip_y(&self) -> (f32, f32) {
		(self.x, -self.y)
	}
}

pub struct HexSpiral {
	center: HexCoord,
	ring_radius: usize,
	ring_iter: Option<Box<dyn Iterator<Item=HexCoord>>>,
}

impl Iterator for HexSpiral {
	type Item = HexCoord;

	fn next(&mut self) -> Option<Self::Item> {
		if self.ring_radius == 0 {
			self.ring_radius = 1;
			self.ring_iter = Some(Box::new(self.center.to_ring(self.ring_radius).into_iter()));
			Some(self.center)
		} else {
			let mut ring_iter = self.ring_iter.take().expect("ring_iter");
			if let Some(item) = ring_iter.next() {
				self.ring_iter = Some(ring_iter);
				Some(item)
			} else {
				self.ring_radius += 1;
				let mut ring = self.center.to_ring(self.ring_radius);
				ring.rotate_right(self.ring_radius - 1);
				let mut ring_iter = Box::new(ring.into_iter());
				let item = ring_iter.next().expect("item in ring");
				self.ring_iter = Some(ring_iter);
				Some(item)
			}
		}
	}
}

impl HexSpiral {
	pub fn new(center: HexCoord) -> Self {
		HexSpiral { center, ring_radius: 0, ring_iter: None }
	}
}

#[cfg(test)]
mod tests {
	use crate::hexgrid::HexCoord;

	#[test]
	fn travel() {
		let tests = [
			HexCoord::default(),
			HexCoord::new(1, -1),
		];
		let mut results = Vec::new();
		for test in tests {
			let pixel = test.to_pixel();
			results.push((pixel.x, pixel.y));
		}
		assert_eq!(vec![(0., 0.), (1.5, -0.8660254037844386)], results);
	}
	#[test]
	fn ring() {
		let center = HexCoord::default();
		let ring = center.to_ring(1);
		assert_eq!(
			&[
				HexCoord::new(1, -1),
				HexCoord::new(1, 0),
				HexCoord::new(0, 1),
				HexCoord::new(-1, 1),
				HexCoord::new(-1, 0),
				HexCoord::new(0, -1),
			],
			ring.as_slice()
		)
	}
	#[test]
	fn spiral() {
		let center = HexCoord::default();
		let spiral = center.to_spiral(1);
		assert_eq!(
			&[
				center,
				HexCoord::new(1, -1),
				HexCoord::new(1, 0),
				HexCoord::new(0, 1),
				HexCoord::new(-1, 1),
				HexCoord::new(-1, 0),
				HexCoord::new(0, -1),
			],
			spiral.as_slice()
		)
	}
	#[test]
	fn endless_spiral() {
		let center = HexCoord::default();
		let spiral_iter = center.iter_spiral();
		let results = spiral_iter.take(9).collect::<Vec<_>>();
		assert_eq!(
			&[
				center,
				HexCoord::new(1, -1),
				HexCoord::new(1, 0),
				HexCoord::new(0, 1),
				HexCoord::new(-1, 1),
				HexCoord::new(-1, 0),
				HexCoord::new(0, -1),
				HexCoord::new(1, -2),
				HexCoord::new(2, -2),
			],
			results.as_slice()
		)
	}
}
