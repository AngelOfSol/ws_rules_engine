use crate::data::CardId;
#[derive(Debug)]
pub struct ZoneState {
    pub content: Vec<CardId>,
}

impl ZoneState {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }
}
