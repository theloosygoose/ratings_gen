use crate::ratings::player_ratings::Ratings;
use crate::ratings::skill_ratings::Skills;
// use crate::generators::gen_ratings::print_many_ratings;
// use crate::generators::constants::{MEAN_RTG, MEAN_STD_DEV, WIDE_STD_DEV};

pub mod ratings;
pub mod generators;
pub mod player;

fn main() {
    let ratings = Ratings::gen_rand();
    let skills = Skills::gen(&ratings);

    let physical_overall = (ratings.burst + ratings.strength + ratings.fluidity + ratings.speed) / 4;
    let offense_overall = (ratings.off_awareness + ratings.shot_form + ratings.ball_handling + ratings.touch) / 4;

    println!("Ratings:: {:#?}", ratings);
    println!("Height:: {:#?}", ratings.height);
    println!("Athleticism Overall:: {:#?}", physical_overall);

    println!("Offense Overall:: {:#?}", offense_overall);

    println!("Skills:: {:#?}", skills);


    // print_many_ratings(MEAN_RTG, MEAN_STD_DEV, WIDE_STD_DEV, 3000);
}
