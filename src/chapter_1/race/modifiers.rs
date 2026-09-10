use super::RaceKind;
use crate::chapter_3::Abilities;

#[derive(Debug, Clone, Default)]
pub struct RaceModifiers {
    pub sub_ability: Abilities,
    pub base_current_armor: i32,
    pub base_life_points: i32,
}

impl From<RaceKind> for RaceModifiers {
    fn from(value: RaceKind) -> Self {
        match value {
            RaceKind::Human => RaceModifiers {
                sub_ability: Abilities::default(),
                base_current_armor: 10,
                base_life_points: 20,
            },
            _ => todo!(),
        }
    }
}

impl RaceModifiers {
    pub fn human() -> Self {
        Self {
            sub_ability: Abilities::default(),
            base_current_armor: 10,
            base_life_points: 20,
        }
    }
}
