pub mod io;

use crate::data::game_data::GameData;
use crate::state::game_state::GameState;
use crate::state::phase::Phase;
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

        self.end_phase();
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

    fn phase_change(&mut self, phase: Phase) {
        self.state.phase = phase;
        self.engine.phase_change(phase, self.state.active_player);
    }

    fn end_phase(&mut self) {
        self.phase_change(Phase::End);
    }
}
