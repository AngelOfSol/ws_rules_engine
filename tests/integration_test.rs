use ws_engine::data::{CardId, Phase};
use ws_engine::rules::io::{ChoiceContext, IO};
use ws_engine::state::player_state::LevelUpResult;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Event {
    Draw {
        card: CardId,
        player: usize,
    },
    Discard {
        card: CardId,
        player: usize,
    },
    Clock {
        card: CardId,
        player: usize,
    },
    PhaseChange {
        phase: Phase,
        player: usize,
    },
    LevelUp {
        result: LevelUpResult,
        player: usize,
    },
}

struct MayAnswer {
    options: Vec<CardId>,
    player: usize,
    context: ChoiceContext,
    value: Option<usize>,
}
struct MustAnswer {
    options: Vec<CardId>,
    player: usize,
    context: ChoiceContext,
    value: usize,
}

struct IntegrationIO {
    events: Vec<Event>,
    may_answers: Vec<MayAnswer>,
    must_answers: Vec<MustAnswer>,
}

impl IO for IntegrationIO {
    fn discard(&mut self, card: CardId, player: usize) {
        assert_eq!(self.events.remove(0), Event::Discard { card, player })
    }
    fn draw(&mut self, card: CardId, player: usize) {
        assert_eq!(self.events.remove(0), Event::Draw { card, player })
    }
    fn clock(&mut self, card: CardId, player: usize) {
        assert_eq!(self.events.remove(0), Event::Clock { card, player })
    }
    fn phase_change(&mut self, phase: Phase, player: usize) {
        assert_eq!(self.events.remove(0), Event::PhaseChange { phase, player })
    }
    fn level_up(&mut self, result: LevelUpResult, player: usize) {
        assert_eq!(self.events.remove(0), Event::LevelUp { result, player })
    }
    fn ask_card_optional_choice(
        &mut self,
        options: &[CardId],
        player: usize,
        context: ChoiceContext,
    ) -> Option<usize> {
        let answer = self.may_answers.remove(0);
        assert_eq!(answer.context, context);
        assert_eq!(answer.player, player);
        assert_eq!(answer.options, options);

        answer.value
    }
    fn ask_card_required_choice(
        &mut self,
        options: &[CardId],
        player: usize,
        context: ChoiceContext,
    ) -> usize {
        let answer = self.must_answers.remove(0);
        assert_eq!(answer.context, context);
        assert_eq!(answer.player, player);
        assert_eq!(answer.options, options);

        answer.value
    }
}
