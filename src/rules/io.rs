use crate::data::{CardId, Phase};
pub trait IO {
    fn phase_change(&mut self, phase: Phase, player: usize);
    fn draw(&mut self, player: usize);

    fn clock(&mut self, card: CardId, player: usize);

    fn ask_choice(&mut self, options: &Vec<CardId>, player: usize) -> Option<CardId>;
}
