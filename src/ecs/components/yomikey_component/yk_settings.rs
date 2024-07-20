#[derive(Debug, Clone)]
pub enum YkeySetting {
	Glyph(usize),
}

pub fn unwrap_or_default(settings: impl AsRef<[YkeySetting]>) -> usize {
	let mut glyph: Option<usize> = None;
	for setting in settings.as_ref() {
		match setting {
			YkeySetting::Glyph(value) => glyph = Some(*value),
		}
	}
	let glyph = glyph.unwrap_or_else(|| 0);
	glyph
}
