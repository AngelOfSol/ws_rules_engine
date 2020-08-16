/// The list of phases a game of Weiss can be in.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Phase {
    Stand,
    End,
    Draw,
    Clock,
}
