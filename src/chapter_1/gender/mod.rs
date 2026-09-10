// Page 156 if roll

use crate::{chapter_1::race::RaceKind, dice::Dice, simulator::Simulator};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn male() -> Self {
        Self::Male
    }

    pub fn female() -> Self {
        Self::Female
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
            Gender::Male
        } else {
            Gender::Female
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_male() {
        assert_eq!(Gender::Male, Gender::male())
    }

    #[test]
    fn get_female() {
        assert_eq!(Gender::Female, Gender::female())
    }
}
