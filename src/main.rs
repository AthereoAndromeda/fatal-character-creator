#![feature(int_roundings)]

mod dice;
mod simulator;
use simulator::*;

use crate::chapter_1::Character;

mod chapter_1;
mod chapter_3;

#[cfg(test)]
const TEST_SEED: u64 = 100;

fn main() {
    let mut sim = GameSimulator::new();

    let character = Character::roll_character(&mut sim);
    dbg!(character);
}

#[cfg(test)]
mod test {
    use crate::{TEST_SEED, TestSimulator};

    #[macro_export]
    macro_rules! set_snapshot_suffix {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(format!($($expr,)*));
        let _guard = settings.bind_to_scope();
    }
}

    #[rstest::fixture]
    pub fn sim() -> TestSimulator {
        TestSimulator::new(TEST_SEED)
    }
}
