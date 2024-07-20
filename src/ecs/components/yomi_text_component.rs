use aframers::components::Width;

use crate::aframe_ex::{Align, Anchor, Baseline, Text};
use crate::views::yomi_data::YomiChar;

pub fn yomi_text(yomi_char: YomiChar) -> Text {
	let text = Text::new()
		.set_align(Align::Center)
		.set_anchor(Anchor::Center)
		.set_baseline(Baseline::Center)
		.set_font(yomi_char.as_font())
		.set_width(Width(1.))
		.set_value(yomi_char.as_glyph())
		.set_wrap_count(1.3)
		;
	text
}
