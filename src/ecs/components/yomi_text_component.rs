use aframers::components::Width;

use crate::aframe_ex::components::baseline_component::Baseline;
use crate::aframe_ex::components::align_component::Align;
use crate::aframe_ex::components::text_component::Text;
use crate::views::yomi_data::YomiChar;

pub fn yomi_text(yomi_char: YomiChar) -> Text {
	let text = Text::new()
		.set_align(Align::Center)
		.set_baseline(Baseline::Center)
		.set_font(yomi_char.as_font())
		.set_width(Width(0.7))
		.set_value(yomi_char.as_glyph())
		.set_wrap_count(2.0)
		;
	text
}
