use std::cell::RefCell;
use std::thread::LocalKey;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::js_sys::Object;

use crate::three_sys::FontLoader;

thread_local! {
	pub static FONT_LOADER: RefCell<FontLoader> = RefCell::new(FontLoader::new());
	pub static KATA_FONT: RefCell<Option<Object>> = RefCell::new(None);
	pub static KANJI_FONT: RefCell<Option<Object>> = RefCell::new(None);
}

const KATA_FONT_URL: &'static str = "assets/typeface/OsakaRegularMonoEnJaRestrictedReversed.json";
const KANJI_FONT_URL: &'static str = "assets/typeface/mplus-1m-light_1m-light.json";

pub fn with_kata_font(f: impl Fn(&Object) + 'static) {
	with_font(KATA_FONT_URL, &KATA_FONT, f);
}
pub fn with_kanji_font(f: impl Fn(&Object) + 'static) {
	with_font(KANJI_FONT_URL, &KANJI_FONT, f);
}

fn with_font(font_url: &str, font_cache: &'static LocalKey<RefCell<Option<Object>>>, f: impl Fn(&Object) + Sized + 'static) {
	let font = font_cache.with_borrow(|font| font.clone());
	match font {
		Some(font) => f(&font),
		None => FONT_LOADER.with_borrow(|loader| {
			loader.load(
				font_url,
				Closure::once_into_js(move |font: &Object| {
					font_cache.set(Some(font.clone()));
					f(font);
				}).unchecked_ref(),
			);
		}),
	}
}
