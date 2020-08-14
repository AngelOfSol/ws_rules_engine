mod data;
mod engine;
mod state;

use engine::Engine;

fn main() {
    let mut engine = Engine::new(());

    for _ in 0..10 {
        engine.run_turn();

        println!("{:?}", engine);
    }
}
