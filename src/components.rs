use aframers::component::core::{Component, component_registration, ComponentValue, register_component};
use aframers::entity::{create_entity, Entity};
use rand::prelude::SliceRandom;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

use crate::ka::parse_kanji;
use crate::more_aframe::{Align, Anchor, Baseline, RingGeometry, Text};

pub struct HexCell;
impl ComponentValue for HexCell {
	fn component_name(&self) -> &str { "hexcell" }

	fn component_value(&self) -> impl AsRef<str> {
		"true"
	}
}

pub fn register_hexcell_component() {
	let kanji = parse_kanji();
	let init = Closure::new(move |hexcell: Component| {
		let kanji = kanji.choose(&mut rand::thread_rng()).expect("kanji");
		//let text_value = "美";
		let ring = ring_entity(&kanji.kanji).expect("make ring");
		let element = hexcell.el();
		element.append_child(ring.element()).expect("append ring");
	});
	let seed = component_registration(&init);
	register_component("hexcell", &seed);
	init.forget();
}

fn ring_entity(text_value: impl AsRef<str>) -> Result<Entity, JsValue> {
	let geometry = RingGeometry::default()
		.set_segments_theta(6)
		.set_radius_outer(1.0)
		;
	let text = Text::new(text_value)
		.set_font("assets/kanjialive-msdf.json")
		.set_wrap_count(1)
		.set_align(Align::Center)
		.set_anchor(Anchor::Center)
		.set_baseline(Baseline::Center)
		;
	let entity = create_entity()?
		.set_component(geometry)?
		.set_component(text)?
		;
	Ok(entity)
}
