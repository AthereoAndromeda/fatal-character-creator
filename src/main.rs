mod simulator;
use rand::RngExt as _;
use simulator::*;


mod chapter_1;

#[cfg(test)]
const TEST_SEED: u64 = 100;

fn main() {
    let mut sim = TestSimulator::new(100);
    let num = sim.rng().random::<u32>();
    println!("{num}");
}

#[cfg(test)]
mod test {
    use crate::{TEST_SEED, TestSimulator};

    #[rstest::fixture]
    pub fn sim() -> TestSimulator {
        TestSimulator::new(TEST_SEED)
    }
}
