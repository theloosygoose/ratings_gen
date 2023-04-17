use crate::ratings;


pub struct Player {
    name: String,
    age: u16,
    ratings: ratings::player_ratings::Ratings,
    skills: ratings::skill_ratings::Skills,
    abilities: ratings::abilities::Abilities,
}
