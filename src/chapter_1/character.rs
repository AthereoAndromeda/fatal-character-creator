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
        let final_abilities = abilities + race.modifiers.sub_ability.clone();

        Self {
            race,
            name,
            gender,
            abilities: final_abilities,
        }
    }
}
