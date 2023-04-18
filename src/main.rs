// use crate::player::Player;
use std::fs;
use crate::generators::gen_name;
// use crate::generators::gen_ratings::print_many_ratings;
// use crate::generators::constants::{MEAN_RTG, MEAN_STD_DEV, WIDE_STD_DEV};

pub mod ratings;
pub mod generators;
pub mod player;


fn main() {
    // let mut players = vec![];
    //
    // let mut n = 0;
    //
    // while n < 100 {
    //     players.push(Player::gen());
    //     
    //     n += 1;
    // }
    // println!("Players: {:#?}", players);
    // 
    //
    gen_name::gen_name();
}
