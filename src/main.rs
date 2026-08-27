mod dice;
mod simulator;
use simulator::*;

use crate::{
    chapter_1::{Character, gender::Gender, race::Race},
    dice::Dice as _,
};

mod chapter_1;

#[cfg(test)]
const TEST_SEED: u64 = 100;

fn main() {
    let mut sim = GameSimulator::new();
    let dice_roll = sim.roll_1d20();
    println!("{dice_roll}");

    let race = Race::roll_random(&mut sim);
    dbg!(race);

    let gender = Gender::roll_random(&mut sim, &race);
    dbg!(gender);

    let character = Character::roll_character(&mut sim);
    dbg!(character);
}

#[cfg(test)]
mod test {
    use crate::{TEST_SEED, TestSimulator};

    #[rstest::fixture]
    pub fn sim() -> TestSimulator {
        TestSimulator::new(TEST_SEED)
    }
}
