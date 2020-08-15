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
    pub fn with_content(content: Vec<CardId>) -> Self {
        Self { content }
    }

    pub fn put_on_top(&mut self, card: CardId) {
        self.content.push(card);
    }

    pub fn take_top(&mut self) -> Option<CardId> {
        self.content.pop()
    }
    pub fn take_bottom(&mut self) -> Option<CardId> {
        if self.content.is_empty() {
            None
        } else {
            Some(self.content.remove(0))
        }
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
        let mut zone = ZoneState::with_content(vec![0.into(), 1.into()]);

        assert_eq!(zone.take_top(), Some(1.into()));
        assert_eq!(zone.take_top(), Some(0.into()));
        assert_eq!(zone.take_top(), None);
    }
    #[test]
    fn put_on_top() {
        let mut zone = ZoneState::with_content(vec![0.into(), 1.into()]);

        zone.put_on_top(2.into());

        assert_eq!(zone.content, vec![0.into(), 1.into(), 2.into()]);

        zone.put_on_top(3.into());

        assert_eq!(zone.content, vec![0.into(), 1.into(), 2.into(), 3.into()]);
    }
    #[test]
    fn take_card_id() {
        let mut zone =
            ZoneState::with_content(vec![0.into(), 1.into(), 2.into(), 3.into(), 4.into()]);
        assert_eq!(zone.take_card_id(1.into()), Some(1.into()));
        assert_eq!(zone.content, vec![0.into(), 2.into(), 3.into(), 4.into()]);

        assert_eq!(zone.take_card_id(1.into()), None);
        assert_eq!(zone.content, vec![0.into(), 2.into(), 3.into(), 4.into()]);

        assert_eq!(zone.take_card_id(3.into()), Some(3.into()));
        assert_eq!(zone.content, vec![0.into(), 2.into(), 4.into()]);
    }
}
