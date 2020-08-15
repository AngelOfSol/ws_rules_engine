use std::io::*;
use std::str::FromStr;
use ws_engine::data::{CardId, Phase};
use ws_engine::rules::{
    io::{ChoiceContext, IO},
    Rules,
};

#[derive(Debug)]
struct BasicIO;

impl IO for BasicIO {
    fn phase_change(&mut self, phase: Phase, turn_player: usize) {
        println!("Phase Changed: {:?} for player {}", phase, turn_player);
    }

    fn draw(&mut self, card: CardId, turn_player: usize) {
        println!("player {} drew a card ({})", turn_player, card);
    }

    fn clock(&mut self, card: CardId, turn_player: usize) {
        println!("player {} clocked card {}", turn_player, card);
    }

    fn ask_card_choice(
        &mut self,
        options: &[CardId],
        choosing_player: usize,
        context: ChoiceContext,
    ) -> Option<usize> {
        let mut choice_buffer = String::new();

        match context {
            ChoiceContext::ClockPhaseCardToClock => {
                println!(
                    "player {} may choose to clock from: {:?}",
                    choosing_player, options
                );
            }
        }

        let _ = stdout().flush();
        let _ = stdin().read_line(&mut choice_buffer);

        let choice_buffer = choice_buffer.trim();

        if choice_buffer == "" {
            None
        } else {
            let id = usize::from_str(&choice_buffer).ok()?.into();
            options.iter().position(|item| *item == id)
        }
    }
}

fn main() {
    let mut io = BasicIO;
    let mut engine = Rules::new();

    for _ in 0..10 {
        engine.run_turn(&mut io);

        println!("{:?}", engine);
    }
}
