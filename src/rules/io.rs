use crate::data::{CardId, Phase};
use crate::state::player_state::LevelUpResult;

/// The context for a given request for user input.  You should render a message based on the value of this enum.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChoiceContext {
    ClockPhaseCardToClock,
    HandLimitDiscard,
    LevelUpProcess,
}

/// A trait describing the way the rules engine will request and provide information to the client.
/// The engine will block until calls to these functions complete.
pub trait IO {
    /// This is called whenever the current phase changes.  
    ///
    /// The `phase` parameter specifies what phase was entered.
    ///
    /// The `player` parameter provides the active player.
    fn phase_change(&mut self, phase: Phase, player: usize);

    /// This is called whenever a player draws a card.
    ///
    /// The `card` parameter specifies which card was drawn.
    ///
    /// The `player` parameter specifies who drew the card (not the active player).

    fn draw(&mut self, card: CardId, player: usize);
    /// This is called whenever a player discards a card.
    ///
    /// The `card` parameter specifies which card was discarded.
    ///
    /// The `player` parameter specifies who drew the card (not the active player).

    fn discard(&mut self, card: CardId, player: usize);
    /// This is called whenever a player levels up.
    ///
    /// The `result` parameter specifies the result of leveling up,
    /// the cards that were sent to the waiting room, and the card that
    /// was put into the level zone.
    ///
    /// The `player` parameter specifies who leveled up (not the active player).
    fn level_up(&mut self, result: LevelUpResult, player: usize);

    /// This is called whenever a player chooses to clock during the Clock phase.
    ///
    /// The `card` parameter specifies which card was put into the clock.
    /// Drawn cards will be available in separate draw events.
    ///
    /// The `player` parameter specifies who leveled up (not the active player).
    fn clock(&mut self, card: CardId, player: usize);

    /// This is called whenever the engine needs the user to make a choice
    // of one card from a set of cards.  The user is not required to choose
    // an option, but if they do it needs to be a valid one.
    ///
    /// The `options` parameter specifies a slice of `CardID`s that the
    /// the requested player should choose from.
    ///
    /// The `player` parameter specifies who should be making the choice.
    ///
    /// The `context` parameter describes the context in which the choice should be made.
    ///
    /// The engine will panic if the return value of this function is not a valid index
    /// into the specified slice.  If no choice was made, this function should return None.
    /// If no `options` is empty, this function should always return None.
    fn ask_card_optional_choice(
        &mut self,
        options: &[CardId],
        player: usize,
        context: ChoiceContext,
    ) -> Option<usize>;

    /// This is called whenever the engine needs the user to make a choice
    // of one card from a set of cards.  The user is required to choose
    // an option, and needs to be a valid one.
    ///
    /// The `options` parameter specifies a slice of `CardID`s that the
    /// the requested player should choose from.
    ///
    /// The `player` parameter specifies who should be making the choice.
    ///
    /// The `context` parameter describes the context in which the choice should be made.
    ///
    /// The engine will panic if the return value of this function is not a valid index
    /// into the specified slice.
    fn ask_card_required_choice(
        &mut self,
        options: &[CardId],
        player: usize,
        context: ChoiceContext,
    ) -> usize;
}

impl IO for () {
    fn phase_change(&mut self, _: Phase, _: usize) {}
    fn draw(&mut self, _: CardId, _: usize) {}
    fn discard(&mut self, _: CardId, _: usize) {}
    fn level_up(&mut self, _: LevelUpResult, _: usize) {}
    fn clock(&mut self, _: CardId, _: usize) {}

    fn ask_card_optional_choice(
        &mut self,
        cards: &[CardId],
        _: usize,
        _: ChoiceContext,
    ) -> Option<usize> {
        if cards.is_empty() {
            None
        } else {
            Some(0)
        }
    }

    fn ask_card_required_choice(&mut self, _: &[CardId], _: usize, _: ChoiceContext) -> usize {
        0
    }
}
