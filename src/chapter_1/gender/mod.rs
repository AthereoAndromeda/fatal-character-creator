// Page 156 if roll

use crate::{chapter_1::race::RaceKind, chapter_3::Abilities, dice::Dice, simulator::Simulator};

#[derive(Debug)]
pub enum Gender {
    Male(Abilities),
    Female(Abilities),
}

impl Gender {
    pub fn male() -> Self {
        Self::Male(Abilities::male())
    }

    pub fn female() -> Self {
        Self::Female(Abilities::female())
    }

    pub fn roll_random(sim: &mut impl Simulator, race: &RaceKind) -> Self {
        let num = sim.roll_1d100();

        let final_num = match race {
            RaceKind::Anakim
            | RaceKind::Ogre
            | RaceKind::CliffOgre
            | RaceKind::GruagachOgre
            | RaceKind::KinderFresserOgre
            | RaceKind::BorbyTroll
            | RaceKind::HillTroll
            | RaceKind::SubTroll => num + 10,

            _ => num,
        };

        if final_num > 52 {
            Gender::Male(Abilities::male())
        } else {
            Gender::Female(Abilities::female())
        }
    }
}

// #[cfg(test)]
// mod test {
//     use crate::chapter_1::gender::Gender;

//     #[test]
//     fn race_modifier() {
//         Gender
//     }
// }
