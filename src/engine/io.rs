use crate::data::CardId;
use crate::state::phase::Phase;
use std::io::*;
use std::str::FromStr;
pub trait IO {
    fn phase_change(&self, phase: Phase, turn_player: usize);
    fn draw(&self, turn_player: usize);

    fn clock(&self, card: CardId, turn_player: usize);

    fn ask_choice(&self, options: &Vec<CardId>, choosing_player: usize) -> Option<CardId>;
}
