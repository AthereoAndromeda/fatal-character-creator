use crate::{dice::Dice, simulator::Simulator};

#[derive(Debug, Clone, Copy)]
pub enum Race {
    Anakim,
    Bugbear,
    BlackDwarf,
    BrownDwarf,
    WhiteDwarf,
    DarkElf,
    LightElf,
    Human,
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
    fn from(value: u64) -> Self {
        match value {
            1 => Self::Anakim,
            2..=16 => Self::Bugbear,
            17..=19 => Self::BlackDwarf,
            20 => Self::BrownDwarf,
            21 => Self::WhiteDwarf,
            22 => Self::DarkElf,
            23 => Self::LightElf,
            24..=53 => Self::Human,
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

pub fn get_race(sim: &mut impl Simulator) -> Race {
    Race::from(sim.roll_1d100())
}
