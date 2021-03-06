use std::io::*;
use std::str::FromStr;
use ws_engine::data::{CardId, Phase};
use ws_engine::rules::io::{ChoiceContext, Input, InterruptChoice, Output, RulesEngineIO};
use ws_engine::rules::Rules;
use ws_engine::state::player_state::LevelUpResult;

#[derive(Debug)]
struct BasicIO;

impl BasicIO {
    fn get_message<T: std::fmt::Debug>(
        options: &[T],
        choosing_player: usize,
        context: ChoiceContext,
        optional: bool,
    ) -> String {
        match context {
            ChoiceContext::ClockPhaseCardToClock => format!(
                "player {} {} choose to clock from: {:?}",
                choosing_player,
                if optional { "may" } else { "must" },
                options
            ),
            ChoiceContext::HandLimitDiscard => format!(
                "player {} {} choose to discard one from: {:?}",
                choosing_player,
                if optional { "may" } else { "must" },
                options
            ),
            ChoiceContext::LevelUpProcess => format!(
                "player {} {} choose from clock to level up with: {:?}",
                choosing_player,
                if optional { "may" } else { "must" },
                options
            ),
            ChoiceContext::InterruptTimingChoice => format!(
                "player {} {}  chooses which to process first among: {:?}",
                choosing_player,
                if optional { "may" } else { "must" },
                options
            ),
        }
    }
}

impl Output for BasicIO {
    fn phase_change(&mut self, phase: Phase, turn_player: usize) {
        println!("Phase Changed: {:?} for player {}", phase, turn_player);
    }

    fn draw(&mut self, card: CardId, turn_player: usize) {
        println!("player {} drew a card ({})", turn_player, card);
    }

    fn discard(&mut self, card: CardId, turn_player: usize) {
        println!("player {} discarded a card ({})", turn_player, card);
    }

    fn level_up(&mut self, result: LevelUpResult, turn_player: usize) {
        println!("player {} leveled up with {:?}", turn_player, result);
    }

    fn clock(&mut self, card: CardId, turn_player: usize) {
        println!("player {} clocked card {}", turn_player, card);
    }

    fn refreshed(&mut self, turn_player: usize) {
        println!("player {} refreshed", turn_player,);
    }
}

impl Input<InterruptChoice> for BasicIO {
    fn ask_optional_choice(
        &mut self,
        options: &[InterruptChoice],
        choosing_player: usize,
        context: ChoiceContext,
    ) -> Option<usize> {
        println!(
            "{}",
            BasicIO::get_message(options, choosing_player, context, true)
        );
        let mut choice_buffer = String::new();

        let _ = stdout().flush();
        let _ = stdin().read_line(&mut choice_buffer);

        let choice_buffer = choice_buffer.trim();

        match choice_buffer {
            "level" => options
                .iter()
                .position(|item| *item == InterruptChoice::Level),
            "refresh" => options
                .iter()
                .position(|item| *item == InterruptChoice::Refresh),
            _ => None,
        }
    }

    fn ask_required_choice(
        &mut self,
        options: &[InterruptChoice],
        choosing_player: usize,
        context: ChoiceContext,
    ) -> usize {
        println!(
            "{}",
            BasicIO::get_message(options, choosing_player, context, false)
        );
        let mut choice_buffer = String::new();

        let _ = stdout().flush();
        let _ = stdin().read_line(&mut choice_buffer);

        loop {
            let _ = stdout().flush();
            let _ = stdin().read_line(&mut choice_buffer);
            match choice_buffer.trim() {
                "level" => {
                    break options
                        .iter()
                        .position(|item| *item == InterruptChoice::Level)
                        .unwrap()
                }
                "refresh" => {
                    break options
                        .iter()
                        .position(|item| *item == InterruptChoice::Refresh)
                        .unwrap()
                }
                _ => (),
            }
        }
    }
}

impl Input<CardId> for BasicIO {
    fn ask_optional_choice(
        &mut self,
        options: &[CardId],
        choosing_player: usize,
        context: ChoiceContext,
    ) -> Option<usize> {
        println!(
            "{}",
            BasicIO::get_message(options, choosing_player, context, true)
        );
        let mut choice_buffer = String::new();

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

    fn ask_required_choice(
        &mut self,
        options: &[CardId],
        choosing_player: usize,
        context: ChoiceContext,
    ) -> usize {
        println!(
            "{}",
            BasicIO::get_message(options, choosing_player, context, false)
        );
        let mut choice_buffer = String::new();

        let _ = stdout().flush();
        let _ = stdin().read_line(&mut choice_buffer);

        let mut choice = choice_buffer.trim();

        while usize::from_str(&choice).is_err() {
            let _ = stdout().flush();
            let _ = stdin().read_line(&mut choice_buffer);
            choice = choice_buffer.trim();
        }
        let id = usize::from_str(&choice).unwrap().into();
        options.iter().position(|item| *item == id).unwrap()
    }
}

impl RulesEngineIO for BasicIO {}

fn main() {
    let mut io = BasicIO;
    let mut engine = Rules::new();

    for _ in 0..50 {
        engine.run_turn(&mut io);

        println!("{:?}", engine);
    }
}
