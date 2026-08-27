use rand::{RngExt, SeedableRng, rngs::ChaCha8Rng};

/// Source of all non-deterministic actions
pub trait Simulator {
    fn rng(&mut self) -> impl RngExt;
}

/// Simulator with PRNG
pub struct TestSimulator {
    rng: ChaCha8Rng,
}

impl TestSimulator {
    pub fn new(seed: u64) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(seed);

        Self { rng }
    }
}

impl Simulator for TestSimulator {
    fn rng(&mut self) -> impl RngExt {
        &mut self.rng
    }
}

#[cfg(test)]
mod test {
    use crate::{simulator::Simulator, test::sim};
    use insta::assert_snapshot;
    use rand::RngExt as _;

    #[rstest::rstest]
    fn sanity_check(mut sim: impl Simulator) {
        let random = sim.rng().random::<u32>();
        assert_snapshot!(random, @"3980652914");

        let random = sim.rng().random::<u32>();
        assert_snapshot!(random, @"2391540005");
    }
}
