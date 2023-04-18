use crate::player::Player;
// use crate::generators::gen_ratings::print_many_ratings;
// use crate::generators::constants::{MEAN_RTG, MEAN_STD_DEV, WIDE_STD_DEV};

pub mod ratings;
pub mod generators;
pub mod player;


fn main() {
    let new_player = Player::gen();

    println!("{:#?}", new_player);
}
