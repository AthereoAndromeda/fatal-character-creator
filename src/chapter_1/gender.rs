// Page 156 if roll

use crate::{chapter_1::race::Race, dice::Dice, simulator::Simulator};

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn roll_random(sim: &mut impl Simulator, race: &Race) -> Gender {
        let num = sim.roll_1d100();

        let final_num = match race {
            Race::Anakim
            | Race::Ogre
            | Race::CliffOgre
            | Race::GruagachOgre
            | Race::KinderFresserOgre
            | Race::BorbyTroll
            | Race::HillTroll
            | Race::SubTroll => num + 10,

            _ => num,
        };

        if final_num > 52 {
            Gender::Male
        } else {
            Gender::Female
        }
    }
}
