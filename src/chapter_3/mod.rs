mod subabilities;
pub use subabilities::*;

use crate::{
    chapter_1::{gender::Gender, race::Race},
    dice::Dice,
    simulator::Simulator,
};

impl std::ops::Add for Abilities {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            physique: self.physique + rhs.physique,
            charisma: self.charisma + rhs.charisma,
            dexterity: self.dexterity + rhs.dexterity,
            intelligence: self.intelligence + rhs.intelligence,
            wisdom: self.wisdom + rhs.wisdom,
            temperament: self.temperament + rhs.temperament,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Abilities {
    pub physique: Physique,
    pub charisma: Charisma,
    pub dexterity: Dexterity,
    pub intelligence: Intelligence,
    pub wisdom: Wisdom,
    pub temperament: Temperament,
}

impl Abilities {
    // NIGHTLY: `.div_floor()` can also be utilized if on nightly branch
    // SAFETY: Caller must guarantee `value` fits in an i32
    unsafe fn calculate_value(value: u64) -> i32 {
        unsafe { (value as f32 / 5.).to_int_unchecked::<i32>() - 1 }
    }

    pub fn male() -> Self {
        Self {
            physique: Physique {
                physical_fitness: 5,
                strength: 30,
                bodily_attractiveness: -3,
                ..Default::default()
            },
            charisma: Charisma {
                facial: -3,
                ..Default::default()
            },
            dexterity: Default::default(),
            intelligence: Intelligence {
                language: -2,
                math: 3,
                analytic: 3,
                ..Default::default()
            },
            wisdom: Wisdom {
                drive: 2,
                intuition: -5,
                reflection: -4,
                ..Default::default()
            },
            temperament: Temperament {
                sanguine: -2,
                choleric: 2,
            },
        }
    }

    pub fn female() -> Self {
        Self {
            physique: Physique {
                physical_fitness: -5,
                strength: -30,
                bodily_attractiveness: 3,
                ..Default::default()
            },
            charisma: Charisma {
                facial: 3,
                ..Default::default()
            },
            dexterity: Default::default(),
            intelligence: Intelligence {
                language: 2,
                math: -3,
                analytic: -3,
                ..Default::default()
            },
            wisdom: Wisdom {
                drive: -2,
                intuition: 5,
                reflection: 4,
                ..Default::default()
            },
            temperament: Temperament {
                sanguine: 2,
                choleric: -2,
            },
        }
    }

    pub fn roll_random(sim: &mut impl Simulator) -> Self {
        // SAFETY: 10d100 roll is guaranteed to fit in an i32, and is not NaN/Infinite
        let mut x = || unsafe { Self::calculate_value(sim.roll_10d100()) };

        let physique = Physique {
            physical_fitness: x(),
            strength: x(),
            bodily_attractiveness: x(),
            health: x(),
        };

        let charisma = Charisma {
            facial: x(),
            vocal: x(),
            kinetic: x(),
            rhetorical: x(),
        };

        let dexterity = Dexterity {
            hand_eye_coordination: x(),
            agility: x(),
            reaction_speed: x(),
            enunciation: x(),
        };

        let intelligence = Intelligence {
            language: x(),
            math: x(),
            analytic: x(),
            spatial: x(),
        };

        let wisdom = Wisdom {
            drive: x(),
            intuition: x(),
            common_sense: x(),
            reflection: x(),
        };

        // TODO: Fix
        let temperament = Temperament {
            sanguine: x(),
            choleric: x(),
        };

        Self {
            physique,
            charisma,
            dexterity,
            intelligence,
            wisdom,
            temperament,
        }
    }

    pub fn apply_modifiers(self, race: &Race, gender: &Gender) -> Self {
        let abilities = self + race.modifiers.sub_ability.clone();

        let modifiers = match gender {
            Gender::Male(m) => m,
            Gender::Female(m) => m,
        };

        // TODO: Modify percentage based
        // abilities + modifiers.clone()
        todo!()
    }
}

#[cfg(test)]
mod test {
    use insta::assert_ron_snapshot;

    use super::*;
    use crate::set_snapshot_suffix;

    #[rstest::rstest]
    #[case(1000)]
    #[case(200)]
    #[case(10)]
    fn abilities_calc(#[case] value: u64) {
        set_snapshot_suffix!("{}", value);
        let value = unsafe { Abilities::calculate_value(value) };
        assert_ron_snapshot!(value);
    }
}
