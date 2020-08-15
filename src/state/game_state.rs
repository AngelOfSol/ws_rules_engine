use crate::data::Phase;
use crate::state::player_state::PlayerState;
#[derive(Debug)]
pub struct GameState {
    pub players: [PlayerState; 2],
    pub active_player: usize,
    pub phase: Phase,
    pub turn: usize,
}

impl GameState {
    pub fn new() -> Self {
        let mut players = [PlayerState::new(), PlayerState::new()];

        for id in 0..50 {
            for player in players.iter_mut() {
                player.deck.put_on_top(id.into());
            }
        }

        Self {
            players,
            active_player: 0,
            phase: Phase::Start,
            turn: 0,
        }
    }

    pub fn active_player(&self) -> usize {
        self.active_player
    }
    pub fn non_active_player(&self) -> usize {
        match self.active_player {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        }
    }
}
