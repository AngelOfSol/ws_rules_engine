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

    pub fn put_on_top(&mut self, card: CardId) {
        self.content.push(card);
    }

    pub fn take_top(&mut self) -> Option<CardId> {
        self.content.pop()
    }

    pub fn take_card_id(&mut self, id: CardId) -> Option<CardId> {
        Some(
            self.content
                .remove(self.content.iter().position(|item| *item == id)?),
        )
    }
}
