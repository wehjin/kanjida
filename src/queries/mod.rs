use kanji_data::examples::KanjiExample;
use kanji_data::KanjiData;

pub fn query_details_at_kanji_point(kanji_point: usize) -> String {
	let kd = KanjiData(kanji_point);
	let mut sections = Vec::new();
	let examples = kd.as_examples();
	for example in examples {
		let KanjiExample { sound, meaning, .. } = example;
		let details = format!("{}: {}\n", sound, meaning);
		sections.push(details);
	}
	let details = sections.join("");
	details
}