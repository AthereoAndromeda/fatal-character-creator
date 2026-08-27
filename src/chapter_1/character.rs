use super::{gender, race};
use crate::{chapter_3::Abilities, simulator::Simulator};

#[derive(Debug)]
pub struct Character {
    race: race::Race,
    gender: gender::Gender,
    abilities: Abilities,
}

impl Character {
    pub fn roll_character(sim: &mut impl Simulator) -> Self {
        let race = race::Race::roll_random(sim);
        let gender = gender::Gender::roll_random(sim, &race.kind);
        let abilities = Abilities::roll_random(sim);
        let final_abilities = abilities + race.modifiers.sub_ability.clone();

        Self {
            race,
            gender,
            abilities: final_abilities,
        }
    }
}
