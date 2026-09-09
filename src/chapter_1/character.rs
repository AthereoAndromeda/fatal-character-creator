use super::{gender, race};
use crate::{chapter_1::name, chapter_3::Abilities, simulator::Simulator};

#[derive(Debug)]
pub struct Character {
    race: race::Race,
    name: name::Name,
    gender: gender::Gender,
    abilities: Abilities,
}

impl Character {
    pub fn roll_character(sim: &mut impl Simulator) -> Self {
        let race = race::Race::roll_random(sim);
        let gender = gender::Gender::roll_random(sim, &race.kind);
        let name = name::Name::roll_random(sim, &race.kind, &gender);

        let abilities = Abilities::roll_random(sim);
        let abilities = abilities.apply_modifiers(&race, &gender);

        Self {
            race,
            name,
            gender,
            abilities,
        }
    }
}
