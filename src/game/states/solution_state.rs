use chrono::{DateTime, Utc};

/// Holds state for a solution.
#[derive(Debug, Copy, Clone)]
pub struct SolutionState {
	pub recent_success: Option<DateTime<Utc>>,
}

impl SolutionState {
	pub fn is_solved(&self) -> bool {
		self.recent_success.is_some()
	}
	pub fn init() -> Self {
		Self { recent_success: None }
	}
	pub fn succeed(self, now: DateTime<Utc>) -> Self {
		let recent_success = Some(now);
		Self { recent_success, ..self }
	}
}