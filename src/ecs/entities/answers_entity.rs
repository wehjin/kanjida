use aframers::components::{Color, Position, Scale, Width};
use aframers::entities::{create_plane_entity, Entity};
use wasm_bindgen::JsValue;

use crate::aframe_ex::{Align, Anchor, Baseline, Text};
use crate::ecs::components::yomikey_component::Yomikey;

pub fn create_answers_panel() -> Result<Entity, JsValue> {
	let mut panel = create_plane_entity()?
		.set_id("answers")?
		.set_component(Color::Web("MintCream".into()))?
		;
	const FONT: &str = "assets/onyofirsts-msdf.json";
	const GLYPHS: [&str; 61] = [
		"ア", "イ", "ウ", "エ", "オ", "カ", "ガ", "キ",
		"ギ", "ク", "グ", "ケ", "ゲ", "コ", "ゴ", "サ",
		"ザ", "シ", "ジ", "ス", "ズ", "セ", "ゼ", "ソ",
		"ゾ", "タ", "ダ", "チ", "ツ", "テ", "デ", "ト",
		"ド", "ナ", "ニ", "ネ", "ノ", "ハ", "バ", "ヒ",
		"ビ", "フ", "ブ", "ヘ", "ベ", "ホ", "ボ", "マ",
		"ミ", "ム", "メ", "モ", "ヤ", "ユ", "ヨ", "ラ",
		"リ", "ル", "レ", "ロ", "ワ"
	];
	let glyphs = &GLYPHS;
	for (i, key_pos) in KeyPos::all().into_iter().enumerate() {
		panel = append_key(panel, key_pos, glyphs.get(i).cloned(), FONT)?;
	}
	Ok(panel)
}

fn text(glyph: &str, font: &str) -> Text {
	let text = Text::new()
		.set_align(Align::Center)
		.set_anchor(Anchor::Center)
		.set_baseline(Baseline::Center)
		.set_font(font)
		.set_width(Width(1.))
		.set_value(glyph)
		.set_wrap_count(1.8)
		;
	text
}
const KEYS_PER_SIDE: usize = 8;
const EDGE_PADDING: f32 = 0.05;
const TWEEN_PADDING: f32 = 0.4 * EDGE_PADDING;
const NUMER: f32 = 1. - 2. * EDGE_PADDING - (KEYS_PER_SIDE as f32 - 1.) * TWEEN_PADDING;
const KEY_SIZE: f32 = NUMER / (KEYS_PER_SIDE as f32);

const SPACING: f32 = KEY_SIZE + TWEEN_PADDING;

fn append_key(panel: Entity, key_pos: KeyPos, glyph: Option<&str>, font: &str) -> Result<Entity, JsValue> {
	let position = {
		let x = -0.5 + EDGE_PADDING + (key_pos.0 as f32 * SPACING) + KEY_SIZE / 2.;
		let y = 0.5 - (EDGE_PADDING + (key_pos.1 as f32 * SPACING) + KEY_SIZE / 2.);
		Position(x, y, 0.01)
	};
	let plain = create_plane_entity()?
		.set_id(key_pos.to_id("yomikey"))?
		.set_component(Yomikey)?
		;
	let decorated = match glyph {
		Some(glyph) => {
			let text = text(glyph, font);
			plain.set_component(text)?
		}
		None => plain,
	};
	let positioned = decorated
		.set_component(Scale(KEY_SIZE, KEY_SIZE, 1.))?
		.set_component(position)?
		;
	panel.append_child(positioned)
}

#[derive(Debug, Copy, Clone)]
struct KeyPos(usize, usize);

impl KeyPos {
	pub fn all() -> Vec<Self> {
		let mut out = Vec::new();
		for v in 0..KEYS_PER_SIDE {
			for u in 0..KEYS_PER_SIDE {
				out.push(Self(u, v));
			}
		}
		out
	}
	pub fn to_id(&self, prefix: impl AsRef<str>) -> String {
		format!("{}-{}-{}", prefix.as_ref(), self.0, self.1)
	}
}
