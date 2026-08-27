pub mod gender;
pub mod race;

use crate::simulator::Simulator;

#[derive(Debug)]
pub struct Character {
    race: race::Race,
    gender: gender::Gender,
}

impl Character {
    pub fn roll_character(sim: &mut impl Simulator) -> Self {
        let race = race::Race::roll_random(sim);
        Self {
            race,
            gender: gender::Gender::roll_random(sim, &race),
        }
    }
}
