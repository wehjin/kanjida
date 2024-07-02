const HEX_SIZE: f32 = 1.0;

#[derive(Copy, Clone, Default, PartialEq)]
pub struct HexCoord {
	pub q: f32,
	pub r: f32,
}

impl HexCoord {
	pub fn new(q: f32, r: f32) -> Self { Self { q, r } }
	pub fn s(&self) -> f32 { -self.q - self.r }
	pub fn to_pixel_flat(&self) -> PixelCoord {
		let x = HEX_SIZE * (3. / 2. * self.q);
		let y = HEX_SIZE * (f32::sqrt(3.) / 2. * self.q + f32::sqrt(3.) * self.r);
		return PixelCoord { x, y };
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

#[cfg(test)]
mod tests {
	use crate::hexgrid::HexCoord;

	#[test]
	pub fn travel() {
		let tests = [
			HexCoord::default(),
			HexCoord::new(1., -1.),
		];
		let mut results = Vec::new();
		for test in tests {
			let pixel = test.to_pixel_flat();
			results.push((pixel.x, pixel.y));
		}
		assert_eq!(vec![(0., 0.), (1.5, -0.8660254037844386)], results);
	}
}
