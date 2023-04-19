use crate::team::teams::Team;

// use crate::generators::gen_ratings::print_many_ratings;
// use crate::generators::constants::{MEAN_RTG, MEAN_STD_DEV, WIDE_STD_DEV};

pub mod ratings;
pub mod generators;
pub mod player;
pub mod team;


fn main() {
    
    let mut teams = Team::gen_teams();

    let team_1 = &mut teams[1];
    println!("{:?}", team_1.players[0]);
    
    team_1.develop_team();

    println!("{:?}", team_1.players[0]);
}
