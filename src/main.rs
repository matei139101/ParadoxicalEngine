use crate::prelude::*;

mod engine;
mod prelude;
mod resources;

fn main() {
    let mut engine = Engine::new();

    engine.setup();
    engine.run();
    engine.stop();
}
