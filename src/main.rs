use crate::{prelude::*};

mod engine;
mod prelude;

fn main() {
    let mut engine = Engine::new();

    engine.setup();
    engine.run();
    engine.stop();
}
