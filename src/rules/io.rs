use crate::data::{CardId, Phase};
pub trait IO {
    fn phase_change(&mut self, phase: Phase, player: usize);
    fn draw(&mut self, player: usize);

    fn clock(&mut self, card: CardId, player: usize);

    fn ask_choice(&mut self, options: &[CardId], player: usize) -> Option<CardId>;
}

impl IO for () {
    fn phase_change(&mut self, _: Phase, _: usize) {}
    fn draw(&mut self, _: usize) {}

    fn clock(&mut self, _: CardId, _: usize) {}

    fn ask_choice(&mut self, _: &[CardId], _: usize) -> Option<CardId> {
        None
    }
}
