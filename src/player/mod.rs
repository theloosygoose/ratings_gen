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
            name: "Player A".to_string(),
            age: 19,
            ratings,
            skills,
        }
    }
}
