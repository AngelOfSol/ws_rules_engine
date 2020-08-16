use crate::data::Phase;
use crate::state::player_state::PlayerState;

/// Holds all of the game state to be used by the rules
/// manager.  Contains state information in regards to
/// who's turn it is and what phase they're in.
#[derive(Debug)]
pub struct GameState {
    pub players: [PlayerState; 2],
    pub active_player: usize,
    pub phase: Phase,
    pub turn: usize,
}

impl GameState {
    /// Creates a default GameState with 50 cards in each players deck.
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
            phase: Phase::Stand,
            turn: 0,
        }
    }

    /// Returns the id of the active player.
    pub fn active_player(&self) -> usize {
        self.active_player
    }
    /// Returns the id of the non active player.
    pub fn non_active_player(&self) -> usize {
        match self.active_player {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        }
    }
}
