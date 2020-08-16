use crate::data::CardId;
use crate::state::zone_state::ZoneState;

pub const MAX_CLOCK_SIZE: usize = 6;

#[derive(Debug)]
pub struct PlayerState {
    pub deck: ZoneState,
    pub hand: ZoneState,
    pub waiting_room: ZoneState,
    pub clock: ZoneState,
    pub level: ZoneState,
    base_hand_limit: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DeckEmpty;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DiscardError {
    EmptyHand,
    InvalidCard,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LevelUpError {
    CannotLevel,
    InvalidCard,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LevelUpResult {
    leveled_card: CardId,
    sent_to_waiting_room: Vec<CardId>,
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

    pub fn exceeding_handlimit(&self) -> bool {
        self.hand.content.len() > self.base_hand_limit
    }

    pub fn needs_to_level(&self) -> bool {
        self.clock.content.len() > MAX_CLOCK_SIZE
    }

    pub fn level_up_with(&mut self, level_up_card: CardId) -> Result<LevelUpResult, LevelUpError> {
        if !self.needs_to_level() {
            return Err(LevelUpError::CannotLevel);
        }
        let mut bottom_clock: Vec<_> = (0..7).map(|_| self.clock.take_bottom().unwrap()).collect();

        if !bottom_clock.contains(&level_up_card) {
            return Err(LevelUpError::InvalidCard);
        }
        let bottom_clock: Vec<_> = bottom_clock
            .drain(..)
            .filter(|card| *card != level_up_card)
            .collect();

        for card in bottom_clock.iter() {
            self.waiting_room.put_on_top(*card);
        }
        self.level.put_on_top(level_up_card);

        Ok(LevelUpResult {
            leveled_card: level_up_card,
            sent_to_waiting_room: bottom_clock,
        })
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            deck: ZoneState::new(),
            hand: ZoneState::new(),
            waiting_room: ZoneState::new(),
            clock: ZoneState::new(),
            level: ZoneState::new(),
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

    #[test]
    fn level_up() {
        let mut player = PlayerState {
            clock: ZoneState::with_content(vec![
                0.into(),
                1.into(),
                2.into(),
                3.into(),
                4.into(),
                5.into(),
                6.into(),
            ]),
            ..Default::default()
        };
        assert_eq!(player.clock.content.len(), 7);
        let starting_waiting_room_size = player.waiting_room.content.len();
        let starting_level = player.level.content.len();

        assert_eq!(
            player.level_up_with(0.into()),
            Ok(LevelUpResult {
                leveled_card: 0.into(),

                sent_to_waiting_room: vec![
                    1.into(),
                    2.into(),
                    3.into(),
                    4.into(),
                    5.into(),
                    6.into(),
                ]
            })
        );

        assert_eq!(*player.level.content.last().unwrap(), 0.into());
        assert_eq!(player.level.content.len(), starting_level + 1);
        assert_eq!(
            player.waiting_room.content.len(),
            starting_waiting_room_size + 6
        );
    }
    #[test]
    fn level_up_invalid_card() {
        let mut player = PlayerState {
            clock: ZoneState::with_content(vec![
                0.into(),
                1.into(),
                2.into(),
                3.into(),
                4.into(),
                5.into(),
                6.into(),
            ]),
            ..Default::default()
        };

        assert_eq!(
            player.level_up_with(8.into()).unwrap_err(),
            LevelUpError::InvalidCard
        );
    }
    #[test]
    fn level_up_cannot_level() {
        let mut player = PlayerState {
            clock: ZoneState::with_content(vec![
                0.into(),
                1.into(),
                2.into(),
                3.into(),
                4.into(),
                5.into(),
            ]),
            ..Default::default()
        };

        assert_eq!(
            player.level_up_with(0.into()).unwrap_err(),
            LevelUpError::CannotLevel
        );
    }
}
