pub mod io;

use crate::data::game_data::GameData;
use crate::state::game_state::GameState;
use crate::state::phase::Phase;
use crate::state::player_state::PlayerState;
use io::IO;
#[derive(Debug)]
pub struct Engine<T> {
    state: GameState,
    data: GameData,
    engine: T,
}

impl<T: IO> Engine<T> {
    pub fn new(interface: T) -> Self {
        Engine {
            state: GameState::new(),
            data: GameData {},
            engine: interface,
        }
    }

    pub fn run_turn(&mut self) {
        self.start_phase();

        self.draw_phase();

        self.clock_phase();

        self.end_phase();
    }
    fn current_player(&self) -> &PlayerState {
        &self.state.players[self.state.active_player]
    }

    fn current_player_mut(&mut self) -> &mut PlayerState {
        &mut self.state.players[self.state.active_player]
    }

    fn start_phase(&mut self) {
        if self.state.active_player == 0 {
            self.state.active_player = 1;
        } else {
            self.state.active_player = 0;
            self.state.turn_number += 1;
        }
        self.phase_change(Phase::Start);
    }

    fn draw_phase(&mut self) {
        self.phase_change(Phase::Draw);
        self.draw_card(self.state.active_player);
    }

    fn clock_phase(&mut self) {
        self.phase_change(Phase::Clock);
        let card = self.engine.ask_choice(
            self.current_player().hand.content(),
            self.state.active_player,
        );

        if card.is_some() {
            let card = card.unwrap();
            let player = self.current_player_mut();
            let card = player.hand.take_card_id(card).unwrap();
            player.clock.put_on_top(card);
            self.engine.clock(card, self.state.active_player);

            self.draw_card(self.state.active_player);
            self.draw_card(self.state.active_player);
        }
    }

    fn draw_card(&mut self, player: usize) {
        let current_player = &mut self.state.players[player];
        let card = current_player.deck.take_top();
        if card.is_some() {
            current_player.hand.put_on_top(card.unwrap());
            self.engine.draw(self.state.active_player);
        }
    }

    fn phase_change(&mut self, phase: Phase) {
        self.state.phase = phase;
        self.engine.phase_change(phase, self.state.active_player);
    }

    fn end_phase(&mut self) {
        self.phase_change(Phase::End);
    }
}
