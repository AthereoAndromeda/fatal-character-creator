//! Subabilities to [`Abilities`](super::Abilities)

use fatal_macros::{GenderModifiers, Summable};

#[derive(Debug, Clone, Copy, Default, Summable, GenderModifiers)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Physique {
    pub physical_fitness: i32,
    pub strength: i32,
    pub bodily_attractiveness: i32,
    pub health: i32,
}

#[derive(Debug, Clone, Copy, Default, Summable, GenderModifiers)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Charisma {
    pub facial: i32,
    pub vocal: i32,
    pub kinetic: i32,
    pub rhetorical: i32,
}

#[derive(Debug, Clone, Copy, Default, Summable, GenderModifiers)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Dexterity {
    pub hand_eye_coordination: i32,
    pub agility: i32,
    pub reaction_speed: i32,
    pub enunciation: i32,
}

#[derive(Debug, Clone, Copy, Default, Summable, GenderModifiers)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Intelligence {
    pub language: i32,
    pub math: i32,
    pub analytic: i32,
    pub spatial: i32,
}

#[derive(Debug, Clone, Copy, Default, Summable, GenderModifiers)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Wisdom {
    pub drive: i32,
    pub intuition: i32,
    pub common_sense: i32,
    pub reflection: i32,
}

// TODO: Add the other temperament modifiers
#[derive(Debug, Clone, Copy, Default, Summable, GenderModifiers)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
/// Describes the mental health and temperament of a character
///
/// # Reference Page
/// Chapter 5: Mind, p. 124
pub struct Temperament {
    pub sanguine: i32,
    pub choleric: i32,
    pub melancholic: i32,
    pub phlegmatic: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{chapter_3::Abilities, set_snapshot_suffix, simulator::Simulator, test::sim};
    use insta::{assert_debug_snapshot, assert_ron_snapshot};

    macro_rules! modifiers_test {
        ($mod_ident:ident, $modifier:expr) => {
            pastey::paste! {
                #[test]
                fn [<gender_modifiers_ $mod_ident>]() {
                    set_snapshot_suffix!("{}", stringify!($mod_ident));
                    let mo = $modifier;

                    let mods = Abilities::male_modifiers().$mod_ident;
                    let new = mo.apply_gender_modifiers(mods);
                    assert_ron_snapshot!(new);

                    let mods = Abilities::female_modifiers().$mod_ident;
                    let new = mo.apply_gender_modifiers(mods);
                    assert_ron_snapshot!(new);
                }
            }
        };
    }

    modifiers_test!(
        physique,
        Physique {
            physical_fitness: 100,
            strength: 100,
            bodily_attractiveness: 100,
            health: 100,
        }
    );

    modifiers_test!(
        charisma,
        Charisma {
            facial: 100,
            kinetic: 100,
            rhetorical: 100,
            vocal: 100,
        }
    );

    modifiers_test!(
        dexterity,
        Dexterity {
            hand_eye_coordination: 100,
            agility: 100,
            reaction_speed: 100,
            enunciation: 100
        }
    );

    modifiers_test!(
        intelligence,
        Intelligence {
            language: 100,
            math: 100,
            analytic: 100,
            spatial: 100
        }
    );

    modifiers_test!(
        wisdom,
        Wisdom {
            drive: 100,
            intuition: 100,
            common_sense: 100,
            reflection: 100
        }
    );

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
            temperament: Temperament {
                sanguine: 95,
                choleric: 92,
                melancholic: 70,
                phlegmatic: 49,
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
