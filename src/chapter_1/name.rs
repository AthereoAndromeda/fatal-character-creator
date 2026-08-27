// Page 947

// TODO: Autogenerate from the PDF
// Required a d1000 roll.

use crate::simulator::Simulator;

pub struct Name {
    first_name: Option<String>,
    last_name: Option<String>,
}

impl Name {
    // TODO: Implement
    pub fn roll_random(sim: &mut impl Simulator) -> Self {
        Self {
            first_name: Some("Katherine".to_string()),
            last_name: Some("Sewell".to_string()),
        }
    }
}
