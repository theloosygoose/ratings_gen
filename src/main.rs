use crate::team::teams::Team;

// use crate::generators::gen_ratings::print_many_ratings;
// use crate::generators::constants::{MEAN_RTG, MEAN_STD_DEV, WIDE_STD_DEV};

pub mod ratings;
pub mod generators;
pub mod player;
pub mod team;


fn main() {
    let teams = Team::gen_teams();

    println!("{:?}", teams[1])

}
