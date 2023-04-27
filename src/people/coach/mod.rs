use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coach{
    wins: u16,
    losses: u16,
}
