pub mod io;

use crate::data::game_data::GameData;
use crate::data::Phase;
use crate::state::game_state::GameState;
use crate::state::player_state::PlayerState;
use io::IO;
#[derive(Debug)]
pub struct Rules {
    state: GameState,
    data: GameData,
}

impl Rules {
    pub fn new() -> Self {
        Rules {
            state: GameState::new(),
            data: GameData {},
        }
    }

    pub fn run_turn<T: IO>(&mut self, io: &mut T) {
        self.start_phase(io);

        self.draw_phase(io);

        self.clock_phase(io);

        self.end_phase(io);
    }
    fn current_player(&self) -> &PlayerState {
        &self.state.players[self.state.active_player]
    }

    fn current_player_mut(&mut self) -> &mut PlayerState {
        &mut self.state.players[self.state.active_player]
    }

    fn start_phase<T: IO>(&mut self, io: &mut T) {
        self.phase_change(io, Phase::Start);
    }

    fn draw_phase<T: IO>(&mut self, io: &mut T) {
        self.phase_change(io, Phase::Draw);
        self.draw_card(io, self.state.active_player);
    }

    fn clock_phase<T: IO>(&mut self, io: &mut T) {
        self.phase_change(io, Phase::Clock);
        let card = io.ask_choice(
            &self.current_player().hand.content,
            self.state.active_player,
        );

        if let Some(card) = card {
            let player = self.current_player_mut();
            let card = player.hand.take_card_id(card).unwrap();
            player.clock.put_on_top(card);
            io.clock(card, self.state.active_player);

            self.draw_card(io, self.state.active_player);
            self.draw_card(io, self.state.active_player);
        }
    }

    fn draw_card<T: IO>(&mut self, io: &mut T, player: usize) {
        let player_state = &mut self.state.players[player];
        player_state
            .draw_card()
            .expect("can't draw a card on an empty deck");
        io.draw(player);
    }

    fn phase_change<T: IO>(&mut self, io: &mut T, phase: Phase) {
        self.state.phase = phase;
        io.phase_change(phase, self.state.active_player);
    }

    fn end_phase<T: IO>(&mut self, io: &mut T) {
        self.phase_change(io, Phase::End);
        self.switch_turns();
    }

    fn switch_turns(&mut self) {
        if self.state.active_player == 0 {
            self.state.active_player = 1;
        } else if self.state.active_player == 1 {
            self.state.active_player = 0;
            self.state.turn += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{CardId, Phase};
    use io::IO;

    #[test]
    fn switch_turns() {
        let mut rules = Rules::new();

        assert_eq!(rules.state.active_player, 0);
        assert_eq!(rules.state.turn, 0);

        rules.switch_turns();

        assert_eq!(rules.state.active_player, 1);
        assert_eq!(rules.state.turn, 0);

        rules.switch_turns();

        assert_eq!(rules.state.active_player, 0);
        assert_eq!(rules.state.turn, 1);

        rules.switch_turns();

        assert_eq!(rules.state.active_player, 1);
        assert_eq!(rules.state.turn, 1);
    }

    #[test]
    fn end_phase() {
        let mut rules = Rules::new();

        assert_eq!(rules.state.active_player, 0);
        assert_eq!(rules.state.turn, 0);

        rules.end_phase(&mut ());

        assert_eq!(rules.state.active_player, 1);
        assert_eq!(rules.state.turn, 0);

        rules.end_phase(&mut ());

        assert_eq!(rules.state.active_player, 0);
        assert_eq!(rules.state.turn, 1);

        rules.end_phase(&mut ());

        assert_eq!(rules.state.active_player, 1);
        assert_eq!(rules.state.turn, 1);
    }

    #[test]
    fn draw_phase() {
        let mut rules = Rules::new();

        let starting_hand_size = rules.current_player().hand.content.len();
        let starting_deck_size = rules.current_player().deck.content.len();

        rules.draw_phase(&mut ());

        assert_eq!(
            rules.current_player().hand.content.len(),
            starting_hand_size + 1
        );
        assert_eq!(
            rules.current_player().deck.content.len(),
            starting_deck_size - 1
        );
    }

    #[test]
    fn clock_phase_no_clock() {
        let mut rules = Rules::new();

        let starting_hand_size = rules.current_player().hand.content.len();
        let starting_deck_size = rules.current_player().deck.content.len();

        rules.clock_phase(&mut ());

        assert_eq!(
            rules.current_player().hand.content.len(),
            starting_hand_size
        );
        assert_eq!(
            rules.current_player().deck.content.len(),
            starting_deck_size
        );
    }
    #[test]
    fn clock_phase_clock() {
        struct ClockTestIO {
            clocked_card: Option<CardId>,
        };

        impl IO for ClockTestIO {
            fn phase_change(&mut self, _: Phase, _: usize) {}
            fn draw(&mut self, _: usize) {}

            fn clock(&mut self, _: CardId, _: usize) {}

            fn ask_choice(&mut self, data: &[CardId], _: usize) -> Option<CardId> {
                self.clocked_card = Some(data[0]);
                self.clocked_card
            }
        }

        let mut io = ClockTestIO { clocked_card: None };

        let mut rules = Rules::new();

        rules.current_player_mut().draw_card().unwrap();

        let starting_hand_size = rules.current_player().hand.content.len();
        let starting_clock_size = rules.current_player().clock.content.len();

        rules.clock_phase(&mut io);

        // plus 2 cards - 1
        assert_eq!(
            rules.current_player().hand.content.len(),
            starting_hand_size + 1
        );
        assert_eq!(
            rules.current_player().clock.content.len(),
            starting_clock_size + 1
        );
        assert_eq!(
            *rules.current_player().clock.content.last().unwrap(),
            io.clocked_card.unwrap()
        );
    }
}
