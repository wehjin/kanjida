use aframers::components::{Color, Position, Scale};
use aframers::entities::{create_box_entity, Entity};

pub mod camera_entity;
pub mod controller_entity;
pub mod ground_entity;
pub mod hexgrid_entity;
pub mod hint_entity;
pub mod keystaff_entity;
pub mod light_entity;
pub mod origin_entity;
pub mod ring_entity;

pub fn create_sprite_entity(position: Position, scale: Option<f32>) -> Entity {
	const SPRITE_SCALE: f32 = 0.6;
	let scale = if let Some(scale) = scale {
		scale
	} else {
		SPRITE_SCALE
	};
	create_box_entity().unwrap()
		.set_component_attribute(Scale(scale, scale, scale)).unwrap()
		.set_component_attribute(position).unwrap()
		.set_component_attribute(Color::Web("tomato".into())).unwrap()
}