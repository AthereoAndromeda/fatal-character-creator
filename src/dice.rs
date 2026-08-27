use rand::RngExt;

use crate::simulator::Simulator;

macro_rules! gen_roll {
    ($( ($amount:expr, $sides:expr) ),+ $(,)?) => {
        pastey::paste! {
            /// Creates a roll function for given dice
            pub trait Dice: Simulator {
                $(
                    fn [<roll_ $amount d $sides>](&mut self) -> u64 {
                        self.rng().random_range((1 * $amount)..=($sides * $amount))
                    }
                )+
            }

            impl<T: Simulator> Dice for T {}

            // Generate a test case for each roll
            #[cfg(test)]
            mod roll_test {
                use insta::*;

                use crate::dice::Dice;
                use crate::simulator::Simulator;
                use crate::test::sim;

                $(
                    #[rstest::rstest]
                    fn [<test_ $amount d $sides>](mut sim: impl Simulator) {
                        let roll = sim.[<roll_ $amount d $sides>]();
                        assert_ron_snapshot!(roll);
                    }

                    #[rstest::rstest]
                    // May or may not take forever
                    fn [<ensure_all_sides_reachable_ $amount d $sides>](mut sim: impl Simulator) {
                        let roll_range = (1 * $amount)..($sides * $amount);
                        let mut set: std::collections::HashSet<u64> = std::collections::HashSet::from_iter(roll_range);

                        loop {
                            let roll = sim.[<roll_ $amount d $sides>]();
                            set.remove(&roll);
                            if set.is_empty() { break };
                        }
                    }
                )+
            }
        }
    }
}

gen_roll!(
    // (N die, sides of die)
    (1, 100),
    (10, 100),
    (1, 1000),
);
