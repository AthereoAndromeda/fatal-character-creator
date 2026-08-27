pub mod gender;
pub mod race;

use crate::{chapter_1::gender::roll_gender, simulator::Simulator};

#[derive(Debug)]
pub struct Character {
    race: race::Race,
    gender: gender::Gender,
}

impl Character {
    pub fn roll_character(sim: &mut impl Simulator) -> Self {
        let race = race::roll_race(sim);
        Self {
            race,
            gender: roll_gender(sim, &race),
        }
    }
}
