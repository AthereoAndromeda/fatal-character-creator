use crate::{dice::Dice, simulator::Simulator};

pub struct Abilities {
    physique: Physique,
    charisma: Charisma,
    dexterity: Dexterity,
    intelligence: Intelligence,
    wisdom: Wisdom,
}

impl Abilities {
    // NIGHTLY: `.div_floor()` can also be utilized if on nightly branch
    // SAFETY: Caller must guarantee `value` fits in an i32
    unsafe fn calculate_value(value: u64) -> i32 {
        unsafe { (value as f32 / 5.).to_int_unchecked::<i32>() - 1 }
    }

    pub fn roll_random(sim: &mut impl Simulator) -> Self {
        // SAFETY: 10d100 roll is guaranteed to fit in an i32, and is not NaN/Infinite
        let value = unsafe { Self::calculate_value(sim.roll_10d100()) };

        todo!()
    }
}

struct Physique {
    physical_fitness: i32,
    strength: i32,
    bodily_attractiveness: i32,
    health: i32,
}

struct Charisma {
    facial: i32,
    vocal: i32,
    kinetic: i32,
    rhetorical: i32,
}

struct Dexterity {
    hand_eye_coordination: i32,
    agility: i32,
    reaction_speed: i32,
    enunciation: i32,
}

struct Intelligence {
    language: i32,
    math: i32,
    analytic: i32,
    spatial: i32,
}

struct Wisdom {
    drive: i32,
    intuition: i32,
    common_sense: i32,
    reflection: i32,
}

#[cfg(test)]
mod test {
    use insta::assert_ron_snapshot;

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
}
