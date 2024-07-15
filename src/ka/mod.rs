use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct KanjiRecord {
	pub kanji: String,
	pub kname: String,
	pub kstroke: u8,
	pub kmeaning: String,
	pub kgrade: String,
	pub kunyomi_ja: String,
	pub kunyomi: String,
	pub onyomi_ja: String,
	pub onyomi: String,
	pub examples: String,
	pub radical: String,
	pub rad_order: f32,
	pub rad_stroke: String,
	pub rad_name_ja: String,
	pub rad_name: String,
	pub rad_meaning: String,
	pub rad_position_ja: String,
	pub rad_position: String,
}

impl KanjiRecord {
	pub fn to_onyomi_ja_vec(&self) -> Vec<String> {
		self.onyomi_ja
			.split("、")
			.map(String::from)
			.collect::<Vec<_>>()
	}
	pub fn to_kunyomi_ja_vec(&self) -> Vec<String> {
		self.kunyomi_ja
			.split("、")
			.map(String::from)
			.collect()
	}
}

pub fn parse_kanji() -> Vec<KanjiRecord> {
	let mut records = vec![];
	let string = include_str!("../../kanji-data-media/language-data/ka_data.csv");
	let mut reader = csv::Reader::from_reader(string.as_bytes());
	for result in reader.deserialize() {
		let record: KanjiRecord = result.expect("record");
		records.push(record);
	}
	records
}

#[cfg(test)]
mod tests {
	use crate::ka::{KanjiRecord, parse_kanji};

	#[test]
	fn onyomi_ja() {
		let kanji = parse_kanji();
		let record = &kanji[4];
		assert_eq!(record.to_onyomi_ja_vec(), vec!["シ", "ジ"])
	}

	#[test]
	fn kunyomi_ja() {
		let kanji = parse_kanji();
		let record = &kanji[4];
		assert_eq!(record.to_kunyomi_ja_vec(), vec!["しめ", "しめす"])
	}

	#[test]
	fn kanji() {
		let kanji = parse_kanji();
		assert_eq!(1235, kanji.len());
		assert_eq!(
			KanjiRecord {
				kanji: "一".to_string(),
				kname: "1".to_string(),
				kstroke: 1,
				kmeaning: "one".to_string(),
				kgrade: "1".to_string(),
				kunyomi_ja: "ひと".to_string(),
				kunyomi: "hito".to_string(),
				onyomi_ja: "イチ".to_string(),
				onyomi: "ichi".to_string(),
				examples: "[ [ \"一年生（いちねんせい）\", \"first-year student\" ], [ \"一番（いちばん）\", \"number one\" ], [ \"一度（いちど）\", \"once\" ], [ \"一杯（いっぱい）\", \"one cup of, a lot of\" ], [ \"一緒（いっしょ）\", \"together\" ], [ \"一分（いっぷん）\", \"one minute\" ], [ \"一枚（いちまい）\", \"one (flat object) \" ], [ \"一つ（ひとつ）\", \"one (object) \" ], [ \"一人（ひとり）\", \"one person\" ] ]".to_string(),
				radical: "⼀".to_string(),
				rad_order: 1.0,
				rad_stroke: "1".to_string(),
				rad_name_ja: "いち".to_string(),
				rad_name: "ichi".to_string(),
				rad_meaning: "one, horizontal stroke".to_string(),
				rad_position_ja: "".to_string(),
				rad_position: "".to_string(),
			}, kanji[0]
		);
	}
}

