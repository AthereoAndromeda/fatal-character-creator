use rand::{RngExt, SeedableRng, rngs::ChaCha8Rng};

trait Simulator {
    fn rng(&mut self) -> impl RngExt;
}

struct TestSimulator {
    rng: ChaCha8Rng,
}

impl TestSimulator {
    fn new(seed: u64) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(seed);

        Self { rng }
    }
}

impl Simulator for TestSimulator {
    fn rng(&mut self) -> impl RngExt {
        &mut self.rng
    }
}

fn main() {
    let mut sim = TestSimulator::new(100);
    let num = sim.rng().random::<u32>();
    println!("{num}");
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;
    use rand::RngExt;

    use crate::{Simulator, TestSimulator};

    #[rstest::fixture]
    fn sim() -> TestSimulator {
        TestSimulator::new(100)
    }

    #[rstest::rstest]
    fn test(mut sim: impl Simulator) {
        let random = sim.rng().random::<u32>();
        assert_snapshot!(random, @"3980652914");
    }
}
