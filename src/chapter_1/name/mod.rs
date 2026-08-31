use crate::{
    chapter_1::{gender::Gender, race::RaceKind},
    dice::Dice,
    simulator::Simulator,
};

pub mod name;

#[derive(Debug, Clone)]
pub struct Name(pub String, pub String);

impl Name {
    pub fn roll_random(sim: &mut impl Simulator, race: &RaceKind, gender: &Gender) -> Self {
        let names_json = name::RaceNamesJSON::new();

        let (first_name, last_name) = match race {
            RaceKind::Anakim | RaceKind::Human => match gender {
                Gender::Male(_) => (
                    names_json
                        .human_male()
                        .get(&sim.roll_1d1000())
                        .unwrap()
                        .to_string(),
                    names_json
                        .human_last()
                        .get(&sim.roll_1d1000())
                        .unwrap()
                        .to_string(),
                ),

                Gender::Female(_) => (
                    names_json
                        .human_female()
                        .get(&sim.roll_1d100())
                        .unwrap()
                        .to_string(),
                    names_json
                        .human_last()
                        .get(&sim.roll_1d1000())
                        .unwrap()
                        .to_string(),
                ),
            },
            _ => todo!(),
        };

        Name(first_name, last_name)
    }
}
