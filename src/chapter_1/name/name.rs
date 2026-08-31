// Page 947

use std::{collections::HashMap, ops::RangeInclusive};

use serde::{Deserialize, Serialize};
// Required a d1000 roll.

#[derive(Debug, Serialize, Deserialize)]
pub struct Affixed {
    pre: Vec<Vec<String>>,
    suf: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HumanNames {
    male: Vec<Vec<String>>,
    female: Vec<Vec<String>>,
    last: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BugbearNames {
    male: Affixed,
    female: Affixed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DwarvenNames {
    male: Vec<Vec<String>>,
    female: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElvenNames {
    male: Vec<Vec<String>>,
    female: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KoboldNames {
    male: Vec<Vec<String>>,
    female: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseOgreNames {
    pre: Vec<Vec<String>>,
    suf: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CliffOgreNames {
    pre: Vec<Vec<String>>,
    suf: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrugachOgreNames {
    pre: Vec<Vec<String>>,
    suf: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KinderFresserNames {
    pre: Vec<Vec<String>>,
    suf: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BorbNames {
    pre: Vec<Vec<String>>,
    suf: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubterraneanTrollNames {
    male: Vec<Vec<String>>,
    female: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RaceNamesJSON {
    human: HumanNames,
    bugbear: BugbearNames,
    dwarven: DwarvenNames,
    elven: ElvenNames,
    kobold: KoboldNames,
    base_ogre: BaseOgreNames,
    cliff_ogre: CliffOgreNames,
    grugach_ogre: GrugachOgreNames,
    kinderfresser: KinderFresserNames,
    borb_hill: BorbNames,
    sub_troll: SubterraneanTrollNames,
}

#[derive(Debug, Clone)]
pub enum EntryIdx {
    Range(RangeInclusive<i32>),
    Int(i32),
}

#[derive(Debug, Clone)]
pub struct NameEntry {
    pub idx: EntryIdx,
    pub name: String,
}

impl RaceNamesJSON {
    pub fn new() -> Self {
        let json = include_str!("./output.json");
        serde_json::from_str(&json).unwrap()
    }
}

macro_rules! parse_json {
    [ $( ($name:expr, $p1:ident.$p2:ident) ),* $(,)?] => {
        pastey::paste! {
            impl RaceNamesJSON {
                $(
                    pub fn $name(&self) -> ::std::collections::HashMap<u64, &str> {
                        let target_vec = &self.$p1.$p2;
                        let mut map = HashMap::new();

                        for entry in target_vec {
                            let idx = &entry[0];
                            let r_name = entry[1].as_str();

                            if let Some(split) = idx.split_once("-") {
                                let start = split.0.parse::<u64>().unwrap();
                                let end = split.1.parse::<u64>().unwrap();

                                (start..=end).for_each(|x| {
                                    map.insert(x, r_name);
                                });
                            } else {
                                map.insert(idx.parse().unwrap(), r_name);
                            };
                        }

                        map
                    }
                )*
            }
        }
    };
}

parse_json![
    (human_male, human.male),
    (human_female, human.female),
    (human_last, human.last),
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_impl() {
        let _ = RaceNamesJSON::new();
    }
}
