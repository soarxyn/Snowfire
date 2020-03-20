mod core;
mod renderer;

use crate::core::{Igniter, Engine};

fn main() {
    let engine = Igniter::ignite()
        .with_title("mononoke".to_string())
        .combust();

    engine.burn();
}
