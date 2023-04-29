#![allow(dead_code)]
pub mod player;
pub mod coach;
pub mod owner;
pub mod scout;
pub mod journalist;

use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::generators::id::generate_person_id;
use crate::generators::name::country::Country;
use crate::generators::name::gen_name::gen_name;

use crate::ratings::tangible::TangibleRatings;
use crate::ratings::intangible::IntangibleRatings;
use crate::ratings::personality::Personality;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Job {
    Coach,
    Owner,
    Player,
    Journalist ,
    Scout,
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

    pub intangibles: IntangibleRatings,
    pub tangibles: TangibleRatings,
}


impl Person {
    pub fn gen_player() -> Person {
        let (name, country) = gen_name();
        let age = rand::thread_rng().gen_range(16..35);
        let job = Job::Player;
        let id = generate_person_id(&name, &country, &age);

        let personality = Personality::gen();
        
        let intangibles = IntangibleRatings::gen();
        let tangibles = TangibleRatings::gen(&intangibles, &personality);

        Person { 
            name, 
            id,
            job, 
            country, 
            age, 
            personality,
            intangibles,
            tangibles,
            
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
