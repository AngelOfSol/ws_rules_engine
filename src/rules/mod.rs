pub mod io;

use crate::data::game_data::GameData;
use crate::data::{CardId, Phase};
use crate::state::game_state::GameState;
use crate::state::player_state::PlayerState;
use io::{ChoiceContext, IO};
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
        self.stand_phase(io);

        self.draw_phase(io);

        self.clock_phase(io);

        self.end_phase(io);
    }
    fn current_player(&self) -> &PlayerState {
        &self.state.players[self.state.active_player]
    }

    #[allow(dead_code)]
    fn current_player_mut(&mut self) -> &mut PlayerState {
        &mut self.state.players[self.state.active_player]
    }

    fn stand_phase<T: IO>(&mut self, io: &mut T) {
        self.phase_change(io, Phase::Stand);
    }

    fn draw_phase<T: IO>(&mut self, io: &mut T) {
        self.phase_change(io, Phase::Draw);
        self.draw_card(io, self.state.active_player);
    }

    fn clock_phase<T: IO>(&mut self, io: &mut T) {
        self.phase_change(io, Phase::Clock);
        let card = io.ask_card_optional_choice(
            &self.current_player().hand.content,
            self.state.active_player,
            ChoiceContext::ClockPhaseCardToClock,
        );

        if let Some(card) = card {
            let card = self.current_player().hand.content[card];
            self.clock_card(io, card, self.state.active_player);
        }
    }

    // precondition: card exists in player's hand
    fn clock_card<T: IO>(&mut self, io: &mut T, card: CardId, player: usize) {
        let player = &mut self.state.players[player];
        let card = player.hand.take_card_id(card).unwrap();
        player.clock.put_on_top(card);
        self.interrupt_type_rules_processing(io);

        io.clock(card, self.state.active_player);

        self.draw_card(io, self.state.active_player);
        self.draw_card(io, self.state.active_player);
    }

    fn draw_card<T: IO>(&mut self, io: &mut T, player: usize) {
        let player_state = &mut self.state.players[player];
        let card = player_state
            .draw_card()
            .expect("can't draw a card on an empty deck");
        io.draw(card, player);
    }

    fn phase_change<T: IO>(&mut self, io: &mut T, phase: Phase) {
        self.state.phase = phase;
        io.phase_change(phase, self.state.active_player);
    }

    fn end_phase<T: IO>(&mut self, io: &mut T) {
        self.phase_change(io, Phase::End);

        self.check_handlimit(io, self.state.active_player);
        //

        self.switch_turns();
    }

    fn check_handlimit<T: IO>(&mut self, io: &mut T, player: usize) {
        let player_state = &mut self.state.players[player];
        while player_state.exceeding_handlimit() {
            let to_discard = io.ask_card_required_choice(
                &player_state.hand.content,
                self.state.active_player,
                ChoiceContext::HandLimitDiscard,
            );
            let to_discard = player_state.hand.content[to_discard];

            player_state.discard_card(to_discard).unwrap();
            io.discard(to_discard, self.state.active_player);
        }
    }

    fn switch_turns(&mut self) {
        if self.state.active_player == 0 {
            self.state.active_player = 1;
        } else if self.state.active_player == 1 {
            self.state.active_player = 0;
            self.state.turn += 1;
        }
    }

    fn interrupt_type_rules_processing<T: IO>(&mut self, io: &mut T) {
        loop {
            let mut done = true;

            for player in [self.state.active_player(), self.state.non_active_player()].iter() {
                if self.state.players[*player].needs_to_level() {
                    done = false;
                    self.level_player(io, *player);
                }
            }

            if done {
                return;
            }
        }
    }

    fn level_player<T: IO>(&mut self, io: &mut T, player: usize) {
        let player_state = &mut self.state.players[player];
        let bottom_clock = &player_state.clock.content[0..7];
        let card_idx =
            io.ask_card_required_choice(bottom_clock, player, ChoiceContext::LevelUpProcess);
        let card = bottom_clock[card_idx];

        let result = player_state.level_up_with(card).unwrap();
        io.level_up(result, player);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let starting_clock_size = rules.current_player().clock.content.len();

        rules.clock_phase(&mut ());

        assert_eq!(
            rules.current_player().hand.content.len(),
            starting_hand_size
        );
        assert_eq!(
            rules.current_player().deck.content.len(),
            starting_deck_size
        );
        assert_eq!(
            rules.current_player().clock.content.len(),
            starting_clock_size
        );
    }
    #[test]
    fn clock_phase_will_clock() {
        let mut rules = Rules::new();

        let starting_hand_size = rules.current_player().hand.content.len();
        let starting_deck_size = rules.current_player().deck.content.len();
        let starting_clock_size = rules.current_player().clock.content.len();

        rules.current_player_mut().draw_card().unwrap();

        rules.clock_phase(&mut ());

        assert_eq!(
            rules.current_player().hand.content.len(),
            starting_hand_size + 2
        );
        assert_eq!(
            rules.current_player().deck.content.len(),
            starting_deck_size - 3
        );
        assert_eq!(
            rules.current_player().clock.content.len(),
            starting_clock_size + 1
        );
    }

    #[test]
    fn clock_card() {
        let mut rules = Rules::new();

        rules.current_player_mut().draw_card().unwrap();

        let starting_hand_size = rules.current_player().hand.content.len();
        let starting_clock_size = rules.current_player().clock.content.len();
        let clocked_card = rules.current_player().hand.content[0];

        rules.clock_card(&mut (), clocked_card, rules.state.active_player);

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
            clocked_card
        );
    }

    #[test]
    fn check_handlimit() {
        let mut rules = Rules::new();

        for _ in 0..10 {
            rules.draw_card(&mut (), rules.state.active_player);
        }
        let starting_hand_size = rules.current_player().hand.content.len();
        let starting_waiting_room_size = rules.current_player().waiting_room.content.len();

        rules.check_handlimit(&mut (), rules.state.active_player);

        assert!(!rules.current_player().exceeding_handlimit());
        assert_eq!(
            rules.current_player().waiting_room.content.len(),
            starting_hand_size - rules.current_player().hand.content.len()
                + starting_waiting_room_size
        );
    }

    #[test]
    fn check_leveling_up() {
        let mut rules = Rules::new();

        for i in 0..14 {
            rules.current_player_mut().clock.put_on_top(i.into());
        }

        let starting_level = rules.current_player().level.content.len();
        let starting_waiting_room_size = rules.current_player().waiting_room.content.len();
        rules.interrupt_type_rules_processing(&mut ());

        assert_eq!(
            rules.current_player().level.content.len(),
            starting_level + 2
        );
        assert_eq!(
            rules.current_player().waiting_room.content.len(),
            starting_waiting_room_size + 6 * 2
        )
    }
}
