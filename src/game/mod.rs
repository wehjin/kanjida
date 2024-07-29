#[cfg(test)]
mod tests;

pub type KanjiPoint = usize;
pub type YomiPoint = usize;
pub type AnswerPoint = usize;
pub type QuizPoint = usize;

pub mod game_material;
pub mod game_view;
pub mod states;

