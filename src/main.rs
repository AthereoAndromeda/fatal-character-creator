mod dice;
mod simulator;
use rand::RngExt as _;
use simulator::*;

use crate::dice::Dice as _;

mod chapter_1;

#[cfg(test)]
const TEST_SEED: u64 = 100;

fn main() {
    let mut sim = TestSimulator::new(100);
    let num = sim.rng().random::<u32>();
    println!("{num}");

    let dice_roll = sim.roll_1d20();
    println!("{dice_roll}")
}

#[cfg(test)]
mod test {
    use crate::{TEST_SEED, TestSimulator};

    #[rstest::fixture]
    pub fn sim() -> TestSimulator {
        TestSimulator::new(TEST_SEED)
    }
}
