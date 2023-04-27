#![allow(dead_code)]
pub mod player;
pub mod coach;
pub mod owner;
pub mod scout;
pub mod journalist;

use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::generators::id::generate_person_id;
use crate::ratings::personality::Personality;
use crate::generators::name::country::Country;

use crate::generators::name::gen_name::gen_name;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Job {
    Coach(coach::Coach),
    Owner(owner::Owner),
    Player(player::Player),
    Journalist(journalist::Journalist) ,
    Scout(scout::Scout),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub id: String,
    
    #[serde(flatten)]
    pub job: Job,
    pub country: Country,
    pub age: u16,

    #[serde(flatten)]
    pub personality: Personality,
}


impl Person {
    pub fn gen_player() -> Person {
        let (name, country) = gen_name();
        let personality = Personality::gen();
        let age = rand::thread_rng().gen_range(16..35);
        let job = Job::Player(player::Player::gen_ratings(&personality));
        let id = generate_person_id(&name, &country, &age);

        Person { 
            name, 
            id,
            job, 
            country, 
            age, 
            personality 
        }
    }

    pub fn gen_coach() -> Person {
        todo!();
    }

    pub fn gen_journalist() -> Person {
        todo!();
    }

    pub fn gen_owner() -> Person{
        todo!();
    }

    pub fn gen_scout() -> Person {
        todo!();
    }
    
}
