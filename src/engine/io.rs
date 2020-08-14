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

impl IO for () {
    fn phase_change(&self, phase: Phase, turn_player: usize) {
        println!("Phase Changed: {:?} for player {}", phase, turn_player);
    }

    fn draw(&self, turn_player: usize) {
        println!("player {} drew a card", turn_player);
    }

    fn clock(&self, card: CardId, turn_player: usize) {
        println!("player {} clocked card {}", turn_player, card);
    }

    fn ask_choice(&self, options: &Vec<CardId>, choosing_player: usize) -> Option<CardId> {
        let mut choice_buffer = String::new();

        println!(
            "player {} may choose to clock from: {:?}",
            choosing_player, options
        );

        let _ = stdout().flush();
        let _ = stdin().read_line(&mut choice_buffer);

        let choice_buffer = choice_buffer.trim();

        if choice_buffer == "" {
            None
        } else {
            let id = CardId::from_str(&choice_buffer).ok()?;
            if options.contains(&id) {
                Some(id)
            } else {
                None
            }
        }
    }
}
