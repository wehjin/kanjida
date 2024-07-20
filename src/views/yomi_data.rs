const FONT: &'static str = "assets/onyonanum-msdf.json";
const GLYPHS: [&'static str; 61] = [
	"ア", "イ", "ウ", "エ", "オ", "カ", "ガ", "キ",
	"ギ", "ク", "グ", "ケ", "ゲ", "コ", "ゴ", "サ",
	"ザ", "シ", "ジ", "ス", "ズ", "セ", "ゼ", "ソ",
	"ゾ", "タ", "ダ", "チ", "ツ", "テ", "デ", "ト",
	"ド", "ナ", "ニ", "ネ", "ノ", "ハ", "バ", "ヒ",
	"ビ", "フ", "ブ", "ヘ", "ベ", "ホ", "ボ", "マ",
	"ミ", "ム", "メ", "モ", "ヤ", "ユ", "ヨ", "ラ",
	"リ", "ル", "レ", "ロ", "ワ"
];

#[derive(Debug, Copy, Clone)]
pub struct YomiChar(usize);

impl From<usize> for YomiChar {
	fn from(value: usize) -> Self { Self(value) }
}

impl YomiChar {
	pub fn is_glyph(&self) -> bool {
		self.0 < GLYPHS.len()
	}
	pub fn as_glyph(&self) -> &'static str {
		GLYPHS.get(self.0).cloned().unwrap_or("")
	}
	pub fn as_font(&self) -> &'static str {
		FONT
	}
}
