use crate::data::CardId;
use crate::state::zone_state::ZoneState;
#[derive(Debug)]
pub struct PlayerState {
    pub deck: ZoneState,
    pub hand: ZoneState,
    pub waiting_room: ZoneState,
    pub clock: ZoneState,
    base_hand_limit: usize,
}

#[derive(Debug)]
pub struct DeckEmpty;

#[derive(Debug)]
pub enum DiscardError {
    EmptyHand,
    InvalidCard,
}

impl PlayerState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn draw_card(&mut self) -> Result<CardId, DeckEmpty> {
        let card = self.deck.take_top().ok_or(DeckEmpty)?;
        self.hand.put_on_top(card);
        Ok(card)
    }

    pub fn discard_card(&mut self, card: CardId) -> Result<(), DiscardError> {
        if self.hand.content.is_empty() {
            return Err(DiscardError::EmptyHand);
        }

        let card = self
            .hand
            .take_card_id(card)
            .ok_or(DiscardError::InvalidCard)?;

        self.waiting_room.put_on_top(card);

        Ok(())
    }

    pub fn check_handlimit(&self) -> bool {
        self.hand.content.len() > self.base_hand_limit
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            deck: ZoneState::new(),
            hand: ZoneState::new(),
            waiting_room: ZoneState::new(),
            clock: ZoneState::new(),
            base_hand_limit: 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::zone_state::ZoneState;

    #[test]
    fn draw_card() {
        let mut player = PlayerState {
            deck: ZoneState::with_content(vec![0.into()]),
            ..Default::default()
        };

        let starting_hand_size = player.hand.content.len();
        let starting_deck_size = player.deck.content.len();

        assert!(player.draw_card().is_ok());

        assert_eq!(player.hand.content.len(), starting_hand_size + 1);
        assert_eq!(player.deck.content.len(), starting_deck_size - 1);
    }
    #[test]
    fn draw_card_empty_deck() {
        let mut player = PlayerState {
            ..Default::default()
        };

        let starting_hand_size = player.hand.content.len();
        let starting_deck_size = player.deck.content.len();

        assert!(player.draw_card().is_err());

        assert_eq!(player.hand.content.len(), starting_hand_size);
        assert_eq!(player.deck.content.len(), starting_deck_size);
    }
}
