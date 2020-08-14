use crate::state::phase::Phase;
use crate::state::player_state::PlayerState;
#[derive(Debug)]
pub struct GameState {
    pub players: [PlayerState; 2],
    pub active_player: usize,
    pub phase: Phase,
    pub turn_number: usize,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: [PlayerState::new(), PlayerState::new()],
            active_player: 1,
            phase: Phase::Start,
            turn_number: 0,
        }
    }
}
