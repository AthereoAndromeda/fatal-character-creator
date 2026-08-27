use rand::RngExt;

#[cfg(test)]
pub mod test_simulator;
#[cfg(test)]
pub use test_simulator::*;

pub mod game_simulator;
pub use game_simulator::*;

/// Source of all non-deterministic actions
pub trait Simulator {
    fn rng(&mut self) -> impl RngExt;
}
