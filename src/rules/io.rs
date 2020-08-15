use crate::data::{CardId, Phase};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChoiceContext {
    ClockPhaseCardToClock,
    HandLimitDiscard,
}

pub trait IO {
    fn phase_change(&mut self, phase: Phase, player: usize);
    fn draw(&mut self, card: CardId, player: usize);
    fn discard(&mut self, card: CardId, player: usize);

    fn clock(&mut self, card: CardId, player: usize);

    fn ask_card_optional_choice(
        &mut self,
        options: &[CardId],
        player: usize,
        context: ChoiceContext,
    ) -> Option<usize>;

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

    fn clock(&mut self, _: CardId, _: usize) {}

    fn ask_card_optional_choice(
        &mut self,
        _: &[CardId],
        _: usize,
        _: ChoiceContext,
    ) -> Option<usize> {
        None
    }

    fn ask_card_required_choice(&mut self, _: &[CardId], _: usize, _: ChoiceContext) -> usize {
        0
    }
}
