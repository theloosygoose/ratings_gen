use crate::people::player::Player;

#[derive(Debug, Clone)]
pub struct Scout {
    bias: String,
    players_scouted: Vec<Player>,
}
