use std::cell::LazyCell;
use std::collections::HashMap;
use std::ops::Deref;

use crate::aframe_ex::components::laser_controls_component::Hand;
use crate::ecs::entities::keystaff_entity::CROWN_DEFAULT_GLYPH;
use crate::ecs::systems::keystaff_system::shield_point::{shield_points_map_glyphs, shield_points_map_glyphs_ignore_floor, ShieldPoint};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ShieldBank {
	A,
	K,
	S,
	T,
	N,
	H,
	M,
	Y,
	R,
}

impl<T: AsRef<str>> From<T> for ShieldBank {
	fn from(value: T) -> Self {
		match value.as_ref() {
			"a" => ShieldBank::A,
			"k" => ShieldBank::K,
			"s" => ShieldBank::S,
			"t" => ShieldBank::T,
			"n" => ShieldBank::N,
			"h" => ShieldBank::H,
			"m" => ShieldBank::M,
			"y" => ShieldBank::Y,
			"r" => ShieldBank::R,
			_ => ShieldBank::N,
		}
	}
}

impl ShieldBank {
	pub fn try_glyph(&self, point: ShieldPoint, hand: Hand) -> Option<&'static str> {
		let glyph_map = match hand {
			Hand::Right => match self {
				ShieldBank::A => &A_GLYPHS,
				ShieldBank::K => &K_GLYPHS,
				ShieldBank::S => &S_GLYPHS,
				ShieldBank::T => &T_GLYPHS,
				ShieldBank::N => &N_GLYPHS,
				ShieldBank::H => &H_GLYPHS,
				ShieldBank::M => &M_GLYPHS,
				ShieldBank::Y => &Y_GLYPHS,
				ShieldBank::R => &R_GLYPHS,
			},
			Hand::Left => &BANK_GLYPHS,
		};
		glyph_map.with(|map| map.deref()[&point])
	}
}

thread_local! {
	static BANK_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs_ignore_floor([
			Some("A"), Some("K"), Some("S"),
			Some("T"), Some("N"), Some("H"),
			Some("M"), Some("Y"), Some( "R"),
		])
	});
	static A_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs_ignore_floor([
			Some( "イ"), Some("ウ"), Some("エ"),
			Some("ア"), Some(CROWN_DEFAULT_GLYPH), Some("オ"),
			None, None, None,
		])
	});
	static K_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			Some( "ギ"), Some("グ"), Some("ゲ"),
			Some("ガ"), Some(CROWN_DEFAULT_GLYPH), Some("ゴ"),
			None, None, None,
			Some( "キ"), Some("ク"), Some("ケ"),
			Some("カ"), Some(CROWN_DEFAULT_GLYPH), Some("コ"),
			None, None, None,
		])
	});
	static S_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			Some( "ジ"), Some("ズ"), Some("ゼ"),
			Some("ザ"), Some(CROWN_DEFAULT_GLYPH), Some("ゾ"),
			None, None, None,
			Some( "シ"), Some("ス"), Some("セ"),
			Some("サ"), Some(CROWN_DEFAULT_GLYPH), Some("ソ"),
			None, None, None,
		])
	});
	static T_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			Some( "ヂ"), Some("ヅ"), Some("デ"),
			Some("ダ"), Some(CROWN_DEFAULT_GLYPH), Some("ド"),
			None, None, None,
			Some( "チ"), Some("ツ"), Some("テ"),
			Some("タ"), Some(CROWN_DEFAULT_GLYPH), Some("ト"),
			None, None, None,
		])
	});
	static N_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs_ignore_floor([
			Some( "ニ"), Some("ヌ"), Some("ネ"),
			Some("ナ"), Some(CROWN_DEFAULT_GLYPH), Some("ノ"),
			None, None, None,
		])
	});
	static H_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			Some( "ビ"), Some("ブ"), Some("ベ"),
			Some("バ"), Some(CROWN_DEFAULT_GLYPH), Some("ボ"),
			None, None, None,
			Some( "ヒ"), Some("フ"), Some("ヘ"),
			Some("ハ"), Some(CROWN_DEFAULT_GLYPH), Some("ホ"),
			None, None, None,
		])
	});
	static M_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs_ignore_floor([
			Some( "ミ"), Some("ム"), Some("メ"),
			Some("マ"), Some(CROWN_DEFAULT_GLYPH), Some("モ"),
			None, None, None,
		])
	});
	static Y_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			Some( "ヷ"), Some("ュ"), Some("ヺ"),
			Some("ャ"), Some(CROWN_DEFAULT_GLYPH), Some("ョ"),
			None, Some("ン"), None,
			Some( "ワ"), Some("ユ"), Some("ヲ"),
			Some("ヤ"), Some(CROWN_DEFAULT_GLYPH), Some("ヨ"),
			None, Some("ン"), None,
		])
	});
	static R_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs_ignore_floor([
			Some( "リ"), Some("ル"), Some("レ"),
			Some("ラ"), Some(CROWN_DEFAULT_GLYPH), Some("ロ"),
			None,  None,  None,
		])
	});
}

