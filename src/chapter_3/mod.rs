use crate::{
    chapter_1::{gender::Gender, race::Race},
    dice::Dice,
    simulator::Simulator,
};
use fatal_macros::Summable;

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

#[derive(Debug, Clone, Default, Summable)]
pub struct Physique {
    pub physical_fitness: i32,
    pub strength: i32,
    pub bodily_attractiveness: i32,
    pub health: i32,
}

#[derive(Debug, Clone, Default, Summable)]
pub struct Charisma {
    pub facial: i32,
    pub vocal: i32,
    pub kinetic: i32,
    pub rhetorical: i32,
}

#[derive(Debug, Clone, Default, Summable)]
pub struct Dexterity {
    pub hand_eye_coordination: i32,
    pub agility: i32,
    pub reaction_speed: i32,
    pub enunciation: i32,
}

#[derive(Debug, Clone, Default, Summable)]
pub struct Intelligence {
    pub language: i32,
    pub math: i32,
    pub analytic: i32,
    pub spatial: i32,
}

#[derive(Debug, Clone, Default, Summable)]
pub struct Wisdom {
    pub drive: i32,
    pub intuition: i32,
    pub common_sense: i32,
    pub reflection: i32,
}

#[derive(Debug, Clone, Default, Summable)]
pub struct Temperament {
    pub sanguine: i32,
    pub choleric: i32,
}

#[cfg(test)]
mod test {
    use insta::{assert_debug_snapshot, assert_ron_snapshot};

    use super::*;
    use crate::set_snapshot_suffix;
    use crate::simulator::Simulator;
    use crate::test::sim;

    #[rstest::rstest]
    #[case(1000)]
    #[case(200)]
    #[case(10)]
    fn abilities_calc(#[case] value: u64) {
        set_snapshot_suffix!("{}", value);
        let value = unsafe { Abilities::calculate_value(value) };
        assert_ron_snapshot!(value);
    }

    #[rstest::rstest]
    fn roll_abilities(mut sim: impl Simulator) {
        let a = Abilities::roll_random(&mut sim);
        assert_debug_snapshot!(a, @"
        Abilities {
            physique: Physique {
                physical_fitness: 111,
                strength: 137,
                bodily_attractiveness: 66,
                health: 192,
            },
            charisma: Charisma {
                facial: 152,
                vocal: 41,
                kinetic: 181,
                rhetorical: 136,
            },
            dexterity: Dexterity {
                hand_eye_coordination: 23,
                agility: 60,
                reaction_speed: 6,
                enunciation: 133,
            },
            intelligence: Intelligence {
                language: 141,
                math: 65,
                analytic: 48,
                spatial: 180,
            },
            wisdom: Wisdom {
                drive: 192,
                intuition: 125,
                common_sense: 194,
                reflection: 131,
            },
        }
        ");

        assert_debug_snapshot!(a.physique.sum(), @"506");
        assert_debug_snapshot!(a.charisma.sum(), @"510");
        assert_debug_snapshot!(a.dexterity.sum(), @"222");
        assert_debug_snapshot!(a.intelligence.sum(), @"434");
        assert_debug_snapshot!(a.wisdom.sum(), @"642");

        assert_debug_snapshot!(a.physique.avg(), @"126");
        assert_debug_snapshot!(a.charisma.avg(), @"127");
        assert_debug_snapshot!(a.dexterity.avg(), @"55");
        assert_debug_snapshot!(a.intelligence.avg(), @"108");
        assert_debug_snapshot!(a.wisdom.avg(), @"160");
    }
}
