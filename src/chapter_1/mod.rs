pub mod gender;
pub mod race;

use crate::{chapter_1::gender::GenderModifiers, chapter_3::Abilities, simulator::Simulator};

#[derive(Debug)]
pub struct Character {
    race: race::Race,
    gender: gender::Gender,
    modifiers: gender::GenderModifiers,
    abilities: Abilities,
}

impl Character {
    pub fn roll_character(sim: &mut impl Simulator) -> Self {
        let race = race::Race::roll_random(sim);
        let gender = gender::Gender::roll_random(sim, &race);
        let modifiers = GenderModifiers::new(&gender);
        let abilities = Abilities::roll_random(sim);

        Self {
            race,
            gender,
            modifiers,
            abilities,
        }
    }
}
