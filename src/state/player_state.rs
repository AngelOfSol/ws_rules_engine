use crate::data::CardId;
use crate::state::zone_state::ZoneState;
#[derive(Debug)]
pub struct PlayerState {
    pub deck: ZoneState,
    pub hand: ZoneState,
    pub clock: ZoneState,
}

#[derive(Debug)]
pub struct DeckEmpty;

impl PlayerState {
    pub fn new() -> Self {
        Self {
            deck: ZoneState::new(),
            hand: ZoneState::new(),
            clock: ZoneState::new(),
        }
    }
    pub fn draw_card(&mut self) -> Result<CardId, DeckEmpty> {
        let card = self.deck.take_top().ok_or(DeckEmpty)?;
        self.hand.put_on_top(card);
        Ok(card)
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
            hand: ZoneState::new(),
            clock: ZoneState::new(),
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
            deck: ZoneState::new(),
            hand: ZoneState::new(),
            clock: ZoneState::new(),
        };

        let starting_hand_size = player.hand.content.len();
        let starting_deck_size = player.deck.content.len();

        assert!(player.draw_card().is_err());

        assert_eq!(player.hand.content.len(), starting_hand_size);
        assert_eq!(player.deck.content.len(), starting_deck_size);
    }
}
