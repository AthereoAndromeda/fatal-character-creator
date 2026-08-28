use crate::{chapter_3::Abilities, dice::Dice as _, simulator::Simulator};

#[derive(Debug, Clone, Default)]
pub struct RaceModifiers {
    sub_ability: Option<Abilities>,
    base_current_armor: i32,
    base_life_points: i32,
}

impl RaceModifiers {
    pub fn human() -> Self {
        Self {
            sub_ability: None,
            base_current_armor: 10,
            base_life_points: 20,
        }
    }
}

#[derive(Debug, Clone, strum::Display)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum Race {
    Anakim,
    Bugbear,
    BlackDwarf,
    BrownDwarf,
    WhiteDwarf,
    DarkElf,
    LightElf,
    Human(RaceModifiers),
    Kobold,
    Ogre,
    CliffOgre,
    GruagachOgre,
    KinderFresserOgre,
    BorbyTroll,
    HillTroll,
    SubTroll,
}

impl From<u64> for Race {
    /// # Panics
    /// Will panic if value is not within range 1..=100
    fn from(value: u64) -> Self {
        match value {
            1 => Self::Anakim,
            2..=16 => Self::Bugbear,
            17..=19 => Self::BlackDwarf,
            20 => Self::BrownDwarf,
            21 => Self::WhiteDwarf,
            22 => Self::DarkElf,
            23 => Self::LightElf,
            24..=53 => Self::Human(RaceModifiers::human()),
            54..=73 => Self::Kobold,
            74..=79 => Self::Ogre,
            80..=81 => Self::CliffOgre,
            82..=84 => Self::GruagachOgre,
            85 => Self::KinderFresserOgre,
            86..=87 => Self::BorbyTroll,
            88..=90 => Self::HillTroll,
            91..=100 => Self::SubTroll,
            _ => unreachable!(),
        }
    }
}

impl Race {
    pub fn roll_random(sim: &mut impl Simulator) -> Self {
        Self::from(sim.roll_1d100())
    }
}

#[cfg(test)]
mod test {
    use super::Race;
    use insta::assert_ron_snapshot;
    use strum::IntoEnumIterator;

    #[rstest::rstest]
    fn test_display_names() {
        for race in Race::iter() {
            let disp = race.to_string();
            assert_ron_snapshot!(disp);
        }
    }

    #[rstest::rstest]
    fn ensure_all_races_reachable() {
        for n in 1..=100 {
            let _ = std::hint::black_box(Race::from(n));
        }
    }

    #[rstest::rstest]
    #[should_panic]
    #[case(1000)]
    #[should_panic]
    #[case(101)]
    #[should_panic]
    #[case(0)]
    fn panic_unreachable(#[case] n: u64) {
        let _ = Race::from(n);
    }
}
