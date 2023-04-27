use serde::{Serialize, Deserialize};

use super::Person;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scout {
    bias: String,
    players_scouted: Vec<Person>,
}
