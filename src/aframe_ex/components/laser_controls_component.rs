use aframers::component::core::ComponentValue;

pub enum Hand {
	Left,
	Right,
}
impl Hand {
	pub fn as_str(&self) -> &str {
		match self {
			Hand::Left => "left",
			Hand::Right => "right",
		}
	}
}

pub struct LaserControls {
	pub hand: Option<Hand>,
}

impl LaserControls {
	pub fn new() -> Self { Self { hand: None } }
	pub fn set_hand(mut self, hand: Hand) -> Self {
		self.hand = Some(hand);
		self
	}
}

impl ComponentValue for LaserControls {
	fn component_name(&self) -> &str { "laser-controls" }

	fn component_value(&self) -> impl AsRef<str> {
		let mut clauses = vec![];
		if let Some(hand) = &self.hand {
			let clause = format!("hand: {}", hand.as_str());
			clauses.push(clause);
		}
		clauses.join("; ")
	}
}
