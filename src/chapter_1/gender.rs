// Page 156 if roll

use crate::{chapter_1::race::RaceKind, dice::Dice, simulator::Simulator};

#[derive(Debug)]
struct PhysiqueModifiers {
    physical_fitness: i32,
    strength: i32,
    bodily_attractiveness: i32,
}

#[derive(Debug)]
struct CharismaModifiers {
    face: i32,
}

#[derive(Debug)]
struct IntelligenceModifiers {
    language: i32,
    math: i32,
    spatial: i32,
}

#[derive(Debug)]
struct WisdomModifiers {
    drive: i32,
    intuition: i32,
    reflection: i32,
}

#[derive(Debug)]
struct TemperanceModifiers {
    sanguine: i32,
    choleric: i32,
}

#[derive(Debug)]
// Percentage-based
pub struct GenderModifiers {
    physique: PhysiqueModifiers,
    charisma: CharismaModifiers,
    intelligence: IntelligenceModifiers,
    wisdom: WisdomModifiers,
    temperament: TemperanceModifiers,
}

impl GenderModifiers {
    pub fn male() -> Self {
        Self {
            physique: PhysiqueModifiers {
                physical_fitness: 5,
                strength: 30,
                bodily_attractiveness: -3,
            },

            charisma: CharismaModifiers { face: -3 },

            intelligence: IntelligenceModifiers {
                language: -2,
                math: 3,
                spatial: 3,
            },

            wisdom: WisdomModifiers {
                drive: 2,
                intuition: -5,
                reflection: -4,
            },

            temperament: TemperanceModifiers {
                sanguine: -2,
                choleric: 2,
            },
        }
    }

    pub fn female() -> Self {
        Self {
            physique: PhysiqueModifiers {
                physical_fitness: -5,
                strength: -30,
                bodily_attractiveness: 3,
            },

            charisma: CharismaModifiers { face: 3 },

            intelligence: IntelligenceModifiers {
                language: 2,
                math: -3,
                spatial: -3,
            },

            wisdom: WisdomModifiers {
                drive: -2,
                intuition: 5,
                reflection: 4,
            },

            temperament: TemperanceModifiers {
                sanguine: 2,
                choleric: -2,
            },
        }
    }
}

#[derive(Debug)]
pub enum Gender {
    Male(GenderModifiers),
    Female(GenderModifiers),
}

impl Gender {
    pub fn male() -> Self {
        Self::Male(GenderModifiers::male())
    }

    pub fn female() -> Self {
        Self::Female(GenderModifiers::female())
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
            Gender::Male(GenderModifiers::male())
        } else {
            Gender::Female(GenderModifiers::female())
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
