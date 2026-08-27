use rand::{RngExt, rngs::StdRng};

pub struct GameSimulator {
    rng: StdRng,
}

impl GameSimulator {
    pub fn new() -> Self {
        Self {
            rng: rand::make_rng(),
        }
    }
}

impl super::Simulator for GameSimulator {
    fn rng(&mut self) -> impl RngExt {
        &mut self.rng
    }
}
