mod dice;
mod simulator;
use rand::RngExt as _;
use simulator::*;

use crate::{chapter_1::get_race, dice::Dice as _};

mod chapter_1;

#[cfg(test)]
const TEST_SEED: u64 = 100;

fn main() {
    let num = sim.rng().random::<u32>();
    println!("{num}");

    let dice_roll = sim.roll_1d20();
    println!("{dice_roll}");

    let race = get_race(&mut sim);
    dbg!(race);
}

#[cfg(test)]
mod test {
    use crate::{TEST_SEED, TestSimulator};

    #[rstest::fixture]
    pub fn sim() -> TestSimulator {
        TestSimulator::new(TEST_SEED)
    }
}
