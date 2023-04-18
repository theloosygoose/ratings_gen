#![allow(dead_code)]
use crate::generators::gen_name::gen_name;
use crate::ratings;


#[derive(Debug)]
pub struct Player {
    name: String,
    age: u16,
    ratings: ratings::player_ratings::Ratings,
    skills: ratings::skill_ratings::Skills,
}

impl Player {
    pub fn gen() -> Player {
        let ratings = ratings::player_ratings::Ratings::gen();
        let skills = ratings::skill_ratings::Skills::gen(&ratings);

        return Player {
            name: gen_name(),
            age: 21,
            ratings,
            skills,
        }
    }
}
