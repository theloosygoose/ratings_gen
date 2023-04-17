// use crate::player::Player;
use std::fs;
use crate::generators::gen_name;
use std::error::Error;
// use crate::generators::gen_ratings::print_many_ratings;
// use crate::generators::constants::{MEAN_RTG, MEAN_STD_DEV, WIDE_STD_DEV};

pub mod ratings;
pub mod generators;
pub mod player;

fn readfile() -> String {
    let string = match fs::read_to_string("../data/nationality.txt") {
        Ok(yes) => yes,
        Err(err) => err.to_string() 
    };
    
    return string;
}

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
    let csv_string = readfile();

    println!("{:#?}", csv_string);

    match gen_name::gen_name() {
        Ok(res) => println!("Read Csv {:?}", res),
        Err(err) => println!("Error Reading CSV {:#?}", err),
    } 

}
