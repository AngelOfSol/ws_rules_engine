use crate::state::phase::Phase;
pub trait IO {
    fn phase_change(&self, phase: Phase, turn_player: usize);
}

impl IO for () {
    fn phase_change(&self, phase: Phase, turn_player: usize) {
        println!("Phase Changed: {:?} for player {}", phase, turn_player);
    }
}
