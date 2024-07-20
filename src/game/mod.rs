#[cfg(test)]
mod tests;

pub type KanjiPoint = usize;
pub type YomiPoint = usize;
pub type AnswerPoint = usize;
pub type QuizPoint = usize;

pub mod answer_state;
pub mod game_state;
pub mod quiz_state;
pub mod solution_state;

