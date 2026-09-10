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

macro_rules! apply_modifiers {
    ($abilities:ident, $modifiers:ident; $($fields:expr),*  $(,)?) => {{
        pastey::paste! {
            Abilities {
                $(
                    $fields: $abilities.$fields.apply_gender_modifiers($modifiers.$fields)
                ),*
            }
        }
    }};
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
/// Character Abilities
pub struct Abilities {
    pub physique: Physique,
    pub charisma: Charisma,
    pub dexterity: Dexterity,
    pub intelligence: Intelligence,
    pub wisdom: Wisdom,
    pub temperament: Temperament,
}

impl Abilities {
    fn calculate_value(value: u64) -> i32 {
        (value.div_floor(5) - 1) as i32
    }

    /// Percentage-based modifiers
    pub fn male_modifiers() -> Self {
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

    /// Percentage-based modifiers
    pub fn female_modifiers() -> Self {
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
        let mut x = || Self::calculate_value(sim.roll_10d100());

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
            Gender::Male => Self::male_modifiers(),
            Gender::Female => Self::female_modifiers(),
        };

        let final_abilities = apply_modifiers!(
            abilities,
            modifiers;
            physique,
            charisma,
            dexterity,
            intelligence,
            wisdom,
            temperament
        );

        final_abilities
    }
}

#[cfg(test)]
mod test {
    use insta::{assert_debug_snapshot, assert_ron_snapshot};

    use super::*;
    use crate::{chapter_1::race::RaceKind, set_snapshot_suffix, test::sim};

    #[rstest::fixture]
    fn ability() -> Abilities {
        Abilities {
            physique: Physique {
                physical_fitness: 100,
                strength: 100,
                bodily_attractiveness: 100,
                health: 100,
            },
            charisma: Charisma {
                facial: 100,
                vocal: 100,
                kinetic: 100,
                rhetorical: 100,
            },
            dexterity: Dexterity {
                hand_eye_coordination: 100,
                agility: 100,
                reaction_speed: 100,
                enunciation: 100,
            },
            intelligence: Intelligence {
                language: 100,
                math: 100,
                analytic: 100,
                spatial: 100,
            },
            wisdom: Wisdom {
                drive: 100,
                intuition: 100,
                common_sense: 100,
                reflection: 100,
            },
            temperament: Temperament {
                sanguine: 100,
                choleric: 100,
            },
        }
    }

    #[rstest::fixture]
    fn ability_101() -> Abilities {
        Abilities {
            physique: Physique {
                physical_fitness: 101,
                strength: 101,
                bodily_attractiveness: 101,
                health: 101,
            },
            charisma: Charisma {
                facial: 101,
                vocal: 101,
                kinetic: 101,
                rhetorical: 101,
            },
            dexterity: Dexterity {
                hand_eye_coordination: 101,
                agility: 101,
                reaction_speed: 101,
                enunciation: 101,
            },
            intelligence: Intelligence {
                language: 101,
                math: 101,
                analytic: 101,
                spatial: 101,
            },
            wisdom: Wisdom {
                drive: 101,
                intuition: 101,
                common_sense: 101,
                reflection: 101,
            },
            temperament: Temperament {
                sanguine: 101,
                choleric: 101,
            },
        }
    }

    #[rstest::rstest]
    #[case(1000)]
    #[case(200)]
    #[case(10)]
    fn abilities_calc(#[case] value: u64) {
        set_snapshot_suffix!("{}", value);
        let value = Abilities::calculate_value(value);
        assert_ron_snapshot!(value);
    }

    #[rstest::rstest]
    #[case(RaceKind::Human)]
    fn apply_modifiers(ability: Abilities, #[case] racekind: RaceKind) {
        set_snapshot_suffix!("{}", racekind);
        let race = Race::from(racekind);
        let a = ability.apply_modifiers(&race, &Gender::Male);
        assert_ron_snapshot!(a);
    }

    #[rstest::rstest]
    #[case(RaceKind::Human)]
    fn apply_modifiers_101(ability_101: Abilities, #[case] racekind: RaceKind) {
        set_snapshot_suffix!("{}", racekind);
        let race = Race::from(racekind);
        let a = ability_101.apply_modifiers(&race, &Gender::Male);
        assert_ron_snapshot!(a);
    }

    #[rstest::rstest]
    fn gender_modifiers_male() {
        let ab = Abilities::male_modifiers();
        assert_debug_snapshot!(ab, @"
        Abilities {
            physique: Physique {
                physical_fitness: 5,
                strength: 30,
                bodily_attractiveness: -3,
                health: 0,
            },
            charisma: Charisma {
                facial: -3,
                vocal: 0,
                kinetic: 0,
                rhetorical: 0,
            },
            dexterity: Dexterity {
                hand_eye_coordination: 0,
                agility: 0,
                reaction_speed: 0,
                enunciation: 0,
            },
            intelligence: Intelligence {
                language: -2,
                math: 3,
                analytic: 3,
                spatial: 0,
            },
            wisdom: Wisdom {
                drive: 2,
                intuition: -5,
                common_sense: 0,
                reflection: -4,
            },
            temperament: Temperament {
                sanguine: -2,
                choleric: 2,
            },
        }
        ");
    }

    #[rstest::rstest]
    fn gender_modifiers_female() {
        let ab = Abilities::female_modifiers();
        assert_debug_snapshot!(ab, @"
        Abilities {
            physique: Physique {
                physical_fitness: -5,
                strength: -30,
                bodily_attractiveness: 3,
                health: 0,
            },
            charisma: Charisma {
                facial: 3,
                vocal: 0,
                kinetic: 0,
                rhetorical: 0,
            },
            dexterity: Dexterity {
                hand_eye_coordination: 0,
                agility: 0,
                reaction_speed: 0,
                enunciation: 0,
            },
            intelligence: Intelligence {
                language: 2,
                math: -3,
                analytic: -3,
                spatial: 0,
            },
            wisdom: Wisdom {
                drive: -2,
                intuition: 5,
                common_sense: 0,
                reflection: 4,
            },
            temperament: Temperament {
                sanguine: 2,
                choleric: -2,
            },
        }
        ");
    }

    #[rstest::rstest]
    fn apply_modifications(mut sim: impl Simulator) {
        let ab = Abilities::roll_random(&mut sim);
        ab.apply_modifiers(&Race::human(), &Gender::Male);
    }
}
