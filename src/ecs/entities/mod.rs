use aframers::components::{Color, Position, Scale};
use aframers::entities::{create_box_entity, Entity};

pub mod answers_entity;
pub mod camera_entity;
pub mod yomigun_entity;
pub mod controller_entity;
pub mod ground_entity;
pub mod hexgrid_entity;
pub mod hint_entity;
pub mod light_entity;
pub mod origin_entity;
pub mod ring_entity;
pub mod sky_entity;

pub fn create_sprite_entity(position: Position) -> Entity {
	const SPRITE_SCALE: f32 = 0.6;
	create_box_entity().unwrap()
		.set_component(Scale(SPRITE_SCALE, SPRITE_SCALE, SPRITE_SCALE)).unwrap()
		.set_component(position).unwrap()
		.set_component(Color::Web("tomato".into())).unwrap()
}