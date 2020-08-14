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
        let mut players = [PlayerState::new(), PlayerState::new()];

        for id in 0..50 {
            for player in 0..2 {
                players[player].deck.put_on_top(id);
            }
        }

        Self {
            players,
            active_player: 1,
            phase: Phase::Start,
            turn_number: 0,
        }
    }
}
