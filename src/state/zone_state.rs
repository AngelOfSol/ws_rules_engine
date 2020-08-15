use crate::data::CardId;
#[derive(Debug)]
pub struct ZoneState {
    content: Vec<CardId>,
}

impl ZoneState {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }
    pub fn with_content(content: Vec<CardId>) -> Self {
        Self { content }
    }

    pub fn content(&self) -> &Vec<CardId> {
        &self.content
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn take_top() {
        let mut zone = ZoneState::with_content(vec![0, 1]);

        assert_eq!(zone.take_top(), Some(1));
        assert_eq!(zone.take_top(), Some(0));
        assert_eq!(zone.take_top(), None);
    }
    #[test]
    fn put_on_top() {
        let mut zone = ZoneState::with_content(vec![0, 1]);

        zone.put_on_top(2);

        assert_eq!(zone.content, vec![0, 1, 2]);

        zone.put_on_top(3);

        assert_eq!(zone.content, vec![0, 1, 2, 3]);
    }
    #[test]
    fn take_card_id() {
        let mut zone = ZoneState::with_content(vec![0, 1, 2, 3, 4]);
        assert_eq!(zone.take_card_id(1), Some(1));
        assert_eq!(zone.content, vec![0, 2, 3, 4]);

        assert_eq!(zone.take_card_id(1), None);
        assert_eq!(zone.content, vec![0, 2, 3, 4]);

        assert_eq!(zone.take_card_id(3), Some(3));
        assert_eq!(zone.content, vec![0, 2, 4]);
    }
}
