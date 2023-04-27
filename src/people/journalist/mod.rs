use serde::{Serialize, Deserialize};

use crate::team::teams::TeamName;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Reigon {
    National,
    TeamName(TeamName),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journalist {
    reigon: Reigon,
    sub_stack: String,
    articles: Vec<String>,
}
