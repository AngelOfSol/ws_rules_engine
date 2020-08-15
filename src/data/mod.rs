pub mod game_data;
mod phase;

pub use phase::Phase;
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CardId(usize);

impl Into<CardId> for usize {
    fn into(self) -> CardId {
        CardId(self)
    }
}

impl std::fmt::Display for CardId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0,)
    }
}
