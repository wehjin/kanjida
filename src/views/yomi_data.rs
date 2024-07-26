//noinspection SpellCheckingInspection
pub const YOMI_FONT: &'static str = "assets/NanumGothicCodingWithXmpls-msdf.json";
pub const YOMI_GLYPHS: [&'static str; 61] = [
	"ア", "イ", "ウ", "エ", "オ", "カ", "ガ", "キ",
	"ギ", "ク", "グ", "ケ", "ゲ", "コ", "ゴ", "サ",
	"ザ", "シ", "ジ", "ス", "ズ", "セ", "ゼ", "ソ",
	"ゾ", "タ", "ダ", "チ", "ツ", "テ", "デ", "ト",
	"ド", "ナ", "ニ", "ネ", "ノ", "ハ", "バ", "ヒ",
	"ビ", "フ", "ブ", "ヘ", "ベ", "ホ", "ボ", "マ",
	"ミ", "ム", "メ", "モ", "ヤ", "ユ", "ヨ", "ラ",
	"リ", "ル", "レ", "ロ", "ワ"
];

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct YomiChar(pub usize);

impl YomiChar {
	pub fn to_code(&self) -> usize {
		self.0
	}
	pub fn is_glyph(&self) -> bool {
		self.0 < YOMI_GLYPHS.len()
	}
	pub fn as_glyph(&self) -> &'static str {
		YOMI_GLYPHS.get(self.0).cloned().unwrap_or("　")
	}
	pub fn as_font(&self) -> &'static str {
		YOMI_FONT
	}
	pub fn to_char(&self) -> char {
		let glyph = self.as_glyph();
		first_char_in_str(glyph)
	}
}

pub fn first_char_in_str(s: &str) -> char {
	s.chars().next().unwrap_or(' ')
}

pub fn split_string_first_char(s: &str) -> (String, String) {
	let mut chars = s.chars();
	let first = chars.next().unwrap_or(' ').to_string();
	let remaining = chars.collect::<String>();
	(first, remaining)
}
