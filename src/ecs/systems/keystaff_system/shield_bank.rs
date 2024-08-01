use std::cell::LazyCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::thread::LocalKey;

use crate::ecs::entities::keystaff_entity::CROWN_DEFAULT_GLYPH;
use crate::ecs::systems::keystaff_system::shield_point::{shield_points_map_glyphs, ShieldPoint};

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
	pub fn try_glyph(&self, point: ShieldPoint) -> Option<&'static str> {
		self.as_glyph_map().with(|map| map.deref()[&point])
	}
	pub fn as_glyph_map(&self) -> &'static LocalKey<LazyCell<HashMap<ShieldPoint, Option<&'static str>>>> {
		match self {
			ShieldBank::A => &A_GLYPHS,
			ShieldBank::K => &K_GLYPHS,
			ShieldBank::S => &S_GLYPHS,
			ShieldBank::T => &T_GLYPHS,
			ShieldBank::N => &N_GLYPHS,
			ShieldBank::H => &H_GLYPHS,
			ShieldBank::M => &M_GLYPHS,
			ShieldBank::Y => &Y_GLYPHS,
			ShieldBank::R => &R_GLYPHS,
		}
	}
}

thread_local! {
	static A_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			None, None, None,
			Some("オ"), Some(CROWN_DEFAULT_GLYPH), Some("ア"),
			Some("エ"), Some("ウ"), Some( "イ"),
		])
	});
	static K_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			None,None,None,
			Some("コ"),Some(CROWN_DEFAULT_GLYPH),Some("カ"),
			Some("ケ"),Some("ク"),Some( "キ"),
		])
	});
	static S_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			None,None,None,
			Some("ソ"),Some(CROWN_DEFAULT_GLYPH),Some("サ"),
			Some("セ"),Some("ス"),Some( "シ"),
		])
	});
	static T_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			None,None,None,
			Some("ト"),Some(CROWN_DEFAULT_GLYPH),Some("タ"),
			Some("テ"),Some("ツ"),Some( "チ"),
		])
	});
	static N_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			None,None,None,
			Some("ノ"),Some(CROWN_DEFAULT_GLYPH),Some("ナ"),
			Some("ネ"),Some("ヌ"),Some( "ニ"),
		])
	});
	static H_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			None,None,None,
			Some("ホ"),Some(CROWN_DEFAULT_GLYPH),Some("ハ"),
			Some("ヘ"),Some("フ"),Some( "ヒ"),
		])
	});
	static M_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			None,None,None,
			Some("モ"),Some(CROWN_DEFAULT_GLYPH),Some("マ"),
			Some("メ"),Some("ム"),Some( "ミ"),
		])
	});
	static Y_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			None,Some("ン"),None,
			Some("ヨ"),Some(CROWN_DEFAULT_GLYPH),Some("ヤ"),
			Some("ヲ"),Some("ユ"),Some( "ワ"),
		])
	});
	static R_GLYPHS : LazyCell<HashMap<ShieldPoint, Option<&'static str>>> = LazyCell::new(||{
		shield_points_map_glyphs([
			None, None, None,
			Some("ロ"), Some(CROWN_DEFAULT_GLYPH), Some("ラ"),
			Some("レ"), Some("ル"), Some( "リ"),
		])
	});
}

