use crate::state::zone_state::ZoneState;
#[derive(Debug)]
pub struct PlayerState {
    pub deck: ZoneState,
    pub hand: ZoneState,
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            deck: ZoneState::new(),
            hand: ZoneState::new(),
        }
    }
}
