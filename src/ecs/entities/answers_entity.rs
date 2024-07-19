use aframers::components::{Color, Position, Scale};
use aframers::entities::{create_plane_entity, Entity};
use wasm_bindgen::JsValue;

pub fn create_answers_panel() -> Result<Entity, JsValue> {
	let panel = create_plane_entity()?
		.set_id("answers")?
		.set_component(Color::Web("MintCream".into()))?
		;
	let panel = KeyPos::all().into_iter().try_fold(panel, append_key)?;
	Ok(panel)
}

const KEYS_PER_SIDE: usize = 8;
const EDGE_PADDING: f32 = 0.03;
const TWEEN_PADDING: f32 = 0.6 * EDGE_PADDING;
const NUMER: f32 = 1. - 2. * EDGE_PADDING - (KEYS_PER_SIDE as f32 - 1.) * TWEEN_PADDING;
const KEY_SIZE: f32 = NUMER / (KEYS_PER_SIDE as f32);
const SPACING: f32 = KEY_SIZE + TWEEN_PADDING;

fn append_key(panel: Entity, key_pos: KeyPos) -> Result<Entity, JsValue> {
	let position = {
		let x = -0.5 + EDGE_PADDING + (key_pos.0 as f32 * SPACING) + KEY_SIZE / 2.;
		let y = 0.5 - (EDGE_PADDING + (key_pos.1 as f32 * SPACING) + KEY_SIZE / 2.);
		Position(x, y, 0.01)
	};
	let key = create_key(key_pos.to_id())?
		.set_component(Scale(KEY_SIZE, KEY_SIZE, 1.))?
		.set_component(position)?
		;
	panel.append_child(key)
}

fn create_key(id: impl AsRef<str>) -> Result<Entity, JsValue> {
	let id = id.as_ref();
	let entity = create_plane_entity()?
		.set_id(id)?
		.set_component(Color::Web("ForestGreen".into()))?
		;
	Ok(entity)
}

#[derive(Debug, Copy, Clone)]
struct KeyPos(usize, usize);

impl KeyPos {
	pub fn all() -> Vec<Self> {
		let mut out = Vec::new();
		for u in 0..KEYS_PER_SIDE {
			for v in 0..KEYS_PER_SIDE {
				out.push(Self(u, v))
			}
		}
		out
	}
	pub fn to_id(&self) -> String {
		format!("an-key-{}-{}", self.0, self.1)
	}
}
